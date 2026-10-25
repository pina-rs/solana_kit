interface ExpectedEngine {
  manifestPath: string;
  sourcePath: string;
  targetName: string;
}

interface ExecutableArtifact {
  executable: string;
}

/** Binds the executable to the actual Cargo package, source, target and build profile. */
export function isEngineArtifact(
  value: unknown,
  expected: ExpectedEngine,
): value is ExecutableArtifact {
  if (!isRecord(value)) {
    return false;
  }

  if (value.reason !== "compiler-artifact" || typeof value.executable !== "string") {
    return false;
  }

  const target = value.target;
  const profile = value.profile;

  if (!isRecord(target) || !isRecord(profile)) {
    return false;
  }

  return value.manifest_path === expected.manifestPath
    && profile.test === false
    && target.name === expected.targetName
    && target.src_path === expected.sourcePath;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}
