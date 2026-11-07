// Build-time utility only; Node is not part of the distributed application.
import { createHash } from "node:crypto";
import { chmod, mkdir, mkdtemp, readFile, rename, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { execFileSync } from "node:child_process";
import { runtimeManifest } from "./runtime-manifest.mjs";

const manifest = runtimeManifest();
const { version, artifact, archiveSha256, executableSha256 } = manifest;
const url = `https://github.com/openai/codex/releases/download/rust-v${version}/${artifact}`;
const destination = resolve(dirname(fileURLToPath(import.meta.url)), "../src-tauri/runtime");
const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");
await mkdir(destination, { recursive: true });
try {
  if (sha256(await readFile(join(destination, "codex"))) === executableSha256) {
    console.log(`Runtime Codex ${version}: artefato verificado, já disponível.`);
    process.exit(0);
  }
} catch (error) {
  if (error.code !== "ENOENT") throw error;
}
const temporary = await mkdtemp(join(tmpdir(), "coplayer-runtime-"));
try {
  console.log(`Baixando Codex ${version}: ${artifact}`);
  const response = await fetch(url, { signal: AbortSignal.timeout(180_000) });
  if (!response.ok) throw new Error(`Download falhou: HTTP ${response.status}`);
  const archive = Buffer.from(await response.arrayBuffer());
  if (sha256(archive) !== archiveSha256) throw new Error("SHA-256 do arquivo publicado não confere.");
  const archivePath = join(temporary, artifact);
  await writeFile(archivePath, archive);
  const executableName = artifact.replace(/\.tar\.gz$/, "");
  execFileSync("/usr/bin/tar", ["-xzf", archivePath, "-C", temporary, executableName]);
  const executable = join(temporary, executableName);
  if (sha256(await readFile(executable)) !== executableSha256) throw new Error("SHA-256 do executável não confere.");
  await chmod(executable, 0o755);
  // Copy across volumes through a destination-local temporary file, then rename.
  const staged = join(destination, "codex.download");
  await writeFile(staged, await readFile(executable), { mode: 0o755 });
  await rename(staged, join(destination, "codex"));
  console.log(`Runtime Codex ${version} preparado e verificado.`);
} finally {
  await rm(temporary, { recursive: true, force: true });
}
