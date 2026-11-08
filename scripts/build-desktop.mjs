import { fileURLToPath } from "node:url";
import { spawnSync } from "node:child_process";

const bundles = { darwin: "app", linux: "appimage" };
const bundle = bundles[process.platform];
if (!bundle) throw new Error(`Plataforma de build não suportada: ${process.platform}`);
const env = { ...process.env };
if (process.platform === "linux") env.APPIMAGE_EXTRACT_AND_RUN = "1";
const build = spawnSync("tauri", ["build", "--bundles", bundle], { stdio: "inherit", env });
if (build.error) throw build.error;
if (build.status !== 0) process.exit(build.status ?? 1);
const verification = process.platform === "linux" ? "verify-linux-bundle.mjs" : "verify-bundle.mjs";
const check = spawnSync(process.execPath, [fileURLToPath(new URL(verification, import.meta.url))], { stdio: "inherit" });
if (check.error) throw check.error;
process.exit(check.status ?? 1);
