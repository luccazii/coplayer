import { spawnSync } from "node:child_process";
import { mkdir, statfs } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { resolve } from "node:path";

const root = fileURLToPath(new URL("../", import.meta.url));
process.chdir(root);
const output = resolve(process.env.COPLAYER_ARTIFACT_DIR || "artifacts/legion");
await mkdir(output, { recursive: true });
const filesystem = await statfs(root);
const availableGiB = filesystem.bavail * filesystem.bsize / 1024 ** 3;
if (availableGiB < 12) {
  throw new Error(`Apenas ${availableGiB.toFixed(1)} GiB livres. Libere pelo menos 12 GiB no volume do projeto/Docker (15–20 GiB recomendados). Nenhuma limpeza automática será feita.`);
}
function run(args) {
  const result = spawnSync("docker", args, { stdio: "inherit" });
  if (result.error) throw result.error;
  if (result.status !== 0) process.exit(result.status || 1);
}
run(["info", "--format", "Docker: {{.OSType}}/{{.Architecture}}"]);
console.log("Build Linux x86_64 isolado; emulação em Macs ARM64 pode demorar.");
run(["buildx", "build", "--platform", "linux/amd64", "--file", "scripts/linux/Dockerfile", "--output", `type=local,dest=${output}`, "."]);
console.log(`Pacote e roteiro: ${output}. Gaming Mode ainda requer validação no Legion.`);
