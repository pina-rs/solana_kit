import { execFileSync } from "node:child_process";
import { readdirSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const formatter = process.env.MONOSTYLE;
const config = process.env.MONOSTYLE_CONFIG;
const dprint = process.env.DPRINT;
const plugin = process.env.DPRINT_TYPESCRIPT_PLUGIN;

if (!formatter || !config || !dprint || !plugin) {
  throw new Error("Provide the verified formatter, policy, dprint and cached TypeScript plugin");
}

const directory = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const files = readdirSync(resolve(directory, "dist"))
  .filter((name) => /\.(js|ts)$/.test(name))
  .map((name) => resolve(directory, "dist", name));

execFileSync(dprint, [
  "fmt",
  "--no-gitignore",
  "--config",
  resolve(directory, "dprint-generated.json"),
  "--plugins",
  plugin,
  "--",
  ...files,
], { cwd: directory, stdio: "inherit" });

execFileSync(formatter, [
  "-c",
  config,
  "fix",
  "--no-ignore",
  "--include-generated",
  "--rule",
  "readability/blank-line-before-control-flow",
  "--rule",
  "readability/blank-line-after-control-flow",
  "--rule",
  "readability/blank-line-before-return",
  "--rule",
  "readability/group-separation",
  ...files,
], { cwd: directory, stdio: "inherit" });
