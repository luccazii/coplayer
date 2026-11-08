// Build-time verification; the shipped application has no Node dependency.
import { createHash } from "node:crypto";
import { createReadStream } from "node:fs";
import { access, readFile } from "node:fs/promises";
import { execFileSync, spawnSync } from "node:child_process";
import { constants } from "node:fs";
import { resolve } from "node:path";

const manifest = JSON.parse(await readFile(new URL("../src-tauri/runtime-manifest.json", import.meta.url), "utf8"));
if (process.platform !== manifest.platform || process.arch !== manifest.architecture) {
  throw new Error("Verificação disponível somente para o pacote macOS ARM64 fixado.");
}
const bundle = resolve("src-tauri/target/release/bundle/macos/Coplayer.app/Contents");
const executable = `${bundle}/Resources/runtime/codex`;
const hash = createHash("sha256");
for await (const chunk of createReadStream(executable)) hash.update(chunk);
if (hash.digest("hex") !== manifest.executableSha256) throw new Error("O runtime empacotado diverge do manifesto.");
await access(executable, constants.X_OK);
await access(`${bundle}/MacOS/coplayer`, constants.X_OK);
for (const binary of [executable, `${bundle}/MacOS/coplayer`]) {
  const architectures = execFileSync("/usr/bin/lipo", ["-archs", binary], { encoding: "utf8" }).trim().split(/\s+/);
  if (!architectures.includes("arm64")) throw new Error("Executável sem arquitetura ARM64 no pacote.");
}
const plist = JSON.parse(execFileSync("/usr/bin/plutil", ["-convert", "json", "-o", "-", `${bundle}/Info.plist`], { encoding: "utf8" }));
if (plist.CFBundleIdentifier !== "dev.coplayer.desktop" || plist.LSMinimumSystemVersion !== "14.0") {
  throw new Error("Identidade ou versão mínima do pacote inesperada.");
}
execFileSync("/usr/bin/codesign", ["--verify", "--strict", resolve(bundle, "..")]);
const signature = spawnSync("/usr/bin/codesign", ["-dv", "--verbose=2", resolve(bundle, "..")], { encoding: "utf8" });
if (signature.status !== 0 || !signature.stderr.includes("Identifier=dev.coplayer.desktop\n") || signature.stderr.includes("Info.plist=not bound") || signature.stderr.includes("Sealed Resources=none")) {
  throw new Error("Bundle sem assinatura vinculada à identidade e aos recursos do aplicativo.");
}
console.log(`Pacote verificado: assinatura do bundle íntegra, macOS 14+, ARM64, runtime ${manifest.version} íntegro e executável.`);
