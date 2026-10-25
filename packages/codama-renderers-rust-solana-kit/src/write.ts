import { writeRenderMap } from "@codama/fragments";
import { lstatSync } from "node:fs";
import { join, parse, resolve } from "node:path";
import { type RustRenderMap, validatePath } from "./files.js";

/** Updates only generated modules, following Codama's standard writer semantics. */
export function writeFiles(outputDirectory: string, files: RustRenderMap): void {
  if (outputDirectory.trim().length === 0) {
    throw new Error("An output directory is required");
  }

  const destination = resolve(outputDirectory);

  if (destination === parse(destination).root) {
    throw new Error("The filesystem root cannot be a generated directory");
  }

  validateDirectory(destination);

  for (const name of files.keys()) {
    validatePath(name);
    const entry = lstatSync(join(destination, name), { throwIfNoEntry: false });

    if (entry && (!entry.isFile() || entry.isSymbolicLink())) {
      throw new Error("A generated module destination is not a regular file");
    }
  }

  writeRenderMap(files, destination);
}

/** Rejects existing symlink ancestors before the first generated-file write. */
function validateDirectory(destination: string): void {
  const parent = parse(destination).root;
  const segments = destination.slice(parent.length).split(/[\\/]/).filter(Boolean);
  let current = parent;

  for (const segment of segments) {
    current = join(current, segment);
    const entry = lstatSync(current, { throwIfNoEntry: false });

    if (entry && (!entry.isDirectory() || entry.isSymbolicLink())) {
      throw new Error("The generated directory contains a non-directory or symlink");
    }
  }
}
