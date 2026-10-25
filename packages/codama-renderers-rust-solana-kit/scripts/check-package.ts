import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { homedir } from "node:os";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const directory = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const engine = readFileSync(resolve(directory, "engine/renderer.wasm"));
const module = new WebAssembly.Module(Uint8Array.from(engine));

if (WebAssembly.Module.imports(module).some((item) => item.module !== "wasi_snapshot_preview1")) {
  throw new Error("The packaged renderer has an unexpected host import module");
}

for (const prefix of [homedir(), "/Users/", "/home/"]) {
  if (engine.includes(Buffer.from(prefix))) {
    throw new Error("The packaged engine contains a personal filesystem prefix");
  }
}

const text = execFileSync(process.env.npm_execpath ? process.execPath : "npm", [
  ...(process.env.npm_execpath ? [process.env.npm_execpath] : []),
  "pack",
  "--dry-run",
  "--json",
  "--ignore-scripts",
  "--offline",
], { cwd: directory, encoding: "utf8", stdio: ["ignore", "pipe", "inherit"] });
const packs: unknown = JSON.parse(text);

if (!Array.isArray(packs) || packs.length !== 1) {
  throw new Error("Expected one NPM pack inventory");
}

const first: unknown = packs[0];

if (
  typeof first !== "object" || first === null || !("files" in first) || !Array.isArray(first.files)
) {
  throw new Error("NPM pack did not report its actual files");
}

const files = new Set<string>();
const entries: readonly unknown[] = first.files;

for (const item of entries) {
  if (
    typeof item !== "object" || item === null || !("path" in item) || typeof item.path !== "string"
  ) {
    throw new Error("NPM pack returned an invalid path");
  }

  files.add(item.path);
}

for (const required of ["dist/index.js", "dist/index.d.ts", "engine/renderer.wasm", "LICENSE"]) {
  if (!files.has(required)) {
    throw new Error(`NPM pack omitted ${required}`);
  }
}

if ([...files].some((file) => /(^|\/)(src|test|target|node_modules)\//.test(file))) {
  throw new Error("The package contains development source or dependencies");
}
