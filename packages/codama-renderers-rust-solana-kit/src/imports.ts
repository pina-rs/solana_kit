import type { WASI } from "node:wasi";

/** Validates Node's opaque import object against the actual trusted engine imports. */
export function getWasiImports(
  { module, wasi }: { module: WebAssembly.Module; wasi: WASI },
): WebAssembly.Imports {
  const imports = wasi.getImportObject();

  if (!("wasi_snapshot_preview1" in imports)) {
    throw new Error("Node did not provide the preview1 WASI namespace");
  }

  const namespace = imports.wasi_snapshot_preview1;

  if (typeof namespace !== "object" || namespace === null) {
    throw new Error("Node provided an invalid WASI namespace");
  }

  const functions: Record<string, WebAssembly.ImportValue> = {};

  for (const entry of WebAssembly.Module.imports(module)) {
    if (entry.module !== "wasi_snapshot_preview1" || entry.kind !== "function") {
      throw new Error("The renderer requires an unexpected host import");
    }

    const value: unknown = Reflect.get(namespace, entry.name);

    if (typeof value !== "function") {
      throw new Error(`Node did not provide WASI function ${entry.name}`);
    }

    functions[entry.name] = value;
  }

  return { wasi_snapshot_preview1: functions };
}
