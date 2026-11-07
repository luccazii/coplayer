import { readFileSync } from "node:fs";

export function runtimeManifest(platform = process.platform, architecture = process.arch) {
  const manifests = {
    "darwin/arm64": "runtime-manifest.json",
    "linux/x64": "runtime-manifest.linux-x64.json",
  };
  const name = manifests[`${platform}/${architecture}`];
  if (!name) throw new Error(`Runtime não definido para ${platform}/${architecture}.`);
  return JSON.parse(readFileSync(new URL(`../src-tauri/${name}`, import.meta.url), "utf8"));
}
