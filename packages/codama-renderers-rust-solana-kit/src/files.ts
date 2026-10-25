import { addToRenderMap, createRenderMap } from "@codama/fragments";
import type { BaseFragment, RenderMap } from "@codama/fragments";

export type RustRenderMap = RenderMap<BaseFragment>;

interface EngineFile {
  path: string;
  source: string;
}

/** Parses the trusted engine protocol before any output directory is touched. */
export function parseFiles(value: unknown): RustRenderMap {
  if (!Array.isArray(value)) {
    throw new Error("Renderer response must be a file array");
  }

  let files = createRenderMap<BaseFragment>();
  const entries: readonly unknown[] = value;

  for (const item of entries) {
    const file = parseFile(item);

    if (files.has(file.path)) {
      throw new Error("Renderer response contains a duplicate module");
    }

    files = addToRenderMap(files, file.path, Object.freeze({ content: file.source }));
  }

  if (!files.has("mod.rs")) {
    throw new Error("Renderer response lacks mod.rs");
  }

  return files;
}

/** Generated modules are flat Rust files, never arbitrary caller paths. */
export function validatePath(path: string): void {
  if (!/^[a-z][a-z\d_]*\.rs$/.test(path)) {
    throw new Error("Renderer response contains an unsafe module path");
  }
}

function parseFile(value: unknown): EngineFile {
  if (typeof value !== "object" || value === null || !("path" in value) || !("source" in value)) {
    throw new Error("Renderer file must contain path and source");
  }

  if (typeof value.path !== "string" || typeof value.source !== "string") {
    throw new Error("Renderer path and source must be strings");
  }

  if (Object.keys(value).length !== 2) {
    throw new Error("Renderer file contains unknown properties");
  }

  validatePath(value.path);

  return { path: value.path, source: value.source };
}
