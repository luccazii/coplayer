import { createHash } from "node:crypto";
import { createReadStream } from "node:fs";
import { access, mkdtemp, open, readFile, readdir, rm } from "node:fs/promises";
import { constants } from "node:fs";
import { execFileSync } from "node:child_process";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { runtimeManifest } from "./runtime-manifest.mjs";

if (process.platform !== "linux" || process.arch !== "x64") throw new Error("Verifique o AppImage dentro do build Linux x86_64.");
const manifest = runtimeManifest();
const config = JSON.parse(await readFile(new URL("../src-tauri/tauri.conf.json", import.meta.url), "utf8"));
const directory = resolve("src-tauri/target/release/bundle/appimage");
const packages = (await readdir(directory)).filter((name) => name.endsWith(".AppImage"));
if (packages.length !== 1) throw new Error("Esperado exatamente um AppImage no diretório de build.");
const temporary = await mkdtemp(join(tmpdir(), "coplayer-verify-"));
async function verifyElf(path) {
  await access(path, constants.X_OK);
  const file = await open(path, "r");
  const header = Buffer.alloc(20);
  try { await file.read(header, 0, header.length, 0); } finally { await file.close(); }
  if (header.subarray(0, 4).toString("hex") !== "7f454c46" || header[4] !== 2 || header[5] !== 1 || header.readUInt16LE(18) !== 62) {
    throw new Error(`Executável não é ELF x86_64: ${path}`);
  }
}
try {
  execFileSync(join(directory, packages[0]), ["--appimage-extract"], { cwd: temporary, stdio: "ignore" });
  const appDir = join(temporary, "squashfs-root");
  const executable = join(appDir, "usr/bin/coplayer");
  const runtime = join(appDir, "usr/libexec", config.productName, "runtime/codex");
  // A duplicate in usr/lib would be rewritten by linuxdeploy and must not ship.
  try {
    await access(join(appDir, "usr/lib", config.productName, "runtime/codex"));
    throw new Error("Runtime duplicado no diretório de bibliotecas.");
  } catch (error) {
    if (error.code !== "ENOENT") throw error;
  }
  await verifyElf(executable);
  await verifyElf(runtime);
  const hash = createHash("sha256");
  for await (const chunk of createReadStream(runtime)) hash.update(chunk);
  const actualHash = hash.digest("hex");
  if (actualHash !== manifest.executableSha256) throw new Error(`Hash do runtime Linux empacotado diverge do manifesto: ${actualHash}.`);
  const version = execFileSync(runtime, ["--version"], { encoding: "utf8", timeout: 15000 }).trim();
  if (version !== `codex-cli ${manifest.version}`) throw new Error(`Versão inesperada: ${version}`);
  await access(join(appDir, "AppRun"), constants.X_OK);
  console.log(`AppImage verificado: ELF x86_64, runtime ${manifest.version} íntegro e executável. Não comprova interface, assinatura ou Gaming Mode.`);
} finally {
  await rm(temporary, { recursive: true, force: true });
}
