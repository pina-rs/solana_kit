import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

/** Prepare the exact consumer closure without resolving registry metadata. */
export function preparePackedConsumer(options: {
  packageDirectory: string;
  consumer: string;
  archive: string;
  name: string;
  version: string;
  codama: string;
}): void {
  const { packageDirectory, consumer, name, version, codama } = options;
  const archive = `file:${resolve(options.archive)}`;
  const entries = readLock(packageDirectory);
  const rendererDependencies = rootDependencies(entries, name, version);
  const dependencies = { [name]: archive, codama };
  const packages = dependencyClosure(entries, [...Object.keys(rendererDependencies), "codama"]);
  const integrity = createHash("sha512").update(readFileSync(options.archive)).digest("base64");
  packages[""] = { dependencies };
  packages[`node_modules/${name}`] = {
    version,
    resolved: archive,
    integrity: `sha512-${integrity}`,
    dependencies: rendererDependencies,
  };
  const manifest = { private: true, type: "module", dependencies };
  writeFileSync(join(consumer, "package.json"), JSON.stringify(manifest));
  writeFileSync(
    join(consumer, "package-lock.json"),
    JSON.stringify({ lockfileVersion: 3, requires: true, packages }),
  );
}

/** Preserve the owning lock's exact versions, URLs and integrity values. */
function dependencyClosure(entries: Record<string, unknown>, pending: string[]) {
  const packages: Record<string, object> = {};

  for (const dependency of pending) {
    const path = `node_modules/${dependency}`;

    if (path in packages) continue;

    const entry = lockedArtifact(entries[path]);
    packages[path] = { ...entry, dev: false };
    pending.push(...Object.keys(strings(entry.dependencies)));
    pending.push(...Object.keys(strings(entry.optionalDependencies)));
    pending.push(...Object.keys(strings(entry.peerDependencies)));
  }

  return packages;
}

function readLock(directory: string): Record<string, unknown> {
  const lock: unknown = JSON.parse(readFileSync(join(directory, "package-lock.json"), "utf8"));

  if (!object(lock) || lock.lockfileVersion !== 3 || !object(lock.packages)) {
    throw new Error("Packed verification requires the owning version 3 package lock");
  }

  return lock.packages;
}

function rootDependencies(entries: Record<string, unknown>, name: string, version: string) {
  const root = entries[""];

  if (!object(root) || root.name !== name || root.version !== version) {
    throw new Error("Packed verification package identity differs from its lock");
  }

  return strings(root.dependencies);
}

function lockedArtifact(entry: unknown): Record<string, unknown> {
  if (
    !object(entry) || typeof entry.version !== "string"
    || typeof entry.resolved !== "string" || typeof entry.integrity !== "string"
  ) {
    throw new Error("Packed dependency is missing its exact locked artifact");
  }

  return entry;
}

/** Reject unrecognized lock data before it becomes consumer dependencies. */
function strings(value: unknown): Record<string, string> {
  if (value === undefined) return {};

  if (!object(value)) throw new Error("Invalid locked dependency map");

  const entries = Object.entries(value);
  const result: Record<string, string> = {};

  for (const [name, version] of entries) {
    if (typeof version !== "string") throw new Error(`Invalid locked dependency: ${name}`);
    result[name] = version;
  }

  return result;
}

function object(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}
