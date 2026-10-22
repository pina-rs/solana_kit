//! Produces canonical tables whose exact contents are checked against source.

use crate::{
    inventory::Inventory,
    model::{PIN, Result, Source},
};

const UNIT_CAPACITY: usize = 200;
use std::{collections::BTreeMap, fs, path::Path};

pub(crate) const CAPABILITIES: &str = "id\tpackage\tpath\tsymbol\tkind\tline\towner\tphase\tstatus\tevidence\tnotes\tunit\tsha256\tprovenance\n";
pub(crate) const FILES: &str = "package\tpath\tsha256\tlines\tgenerated\tdeclarations\texported\tentrypoints\tparser_coverage\tunresolved\tprovenance\n";

pub(crate) struct Tables {
    pub capabilities: String,
    pub sources: String,
    pub units: String,
    pub counts: BTreeMap<String, usize>,
}

impl Tables {
    /// A declaration is one capability even when getter/setter signatures share it.
    pub fn build(inventory: &Inventory) -> Result<Self> {
        let mut output = Self {
            capabilities: CAPABILITIES.into(),
            sources: FILES.into(),
            units: units_header(),
            counts: BTreeMap::new(),
        };

        for package in &inventory.packages {
            let mut entries = BTreeMap::new();

            for source in inventory
                .sources
                .values()
                .filter(|source| source.package == package.name)
            {
                output.sources.push_str(&file_row(source));

                collect_entries(source, &mut entries);
            }

            for (index, (id, (source, declaration))) in entries.iter().enumerate() {
                let unit = format!("{}-{:03}", package.name, index / UNIT_CAPACITY + 1);
                let (status, evidence, notes) = pilot(&source.package, &declaration.symbol);
                let provenance = format!("{PIN}:{}", source.path);
                let fields = [
                    id.clone(),
                    package.name.clone(),
                    source.path.clone(),
                    declaration.symbol.clone(),
                    source
                        .exported
                        .iter()
                        .filter(|item| item.symbol == declaration.symbol)
                        .map(|item| item.kind.as_str())
                        .collect::<std::collections::BTreeSet<_>>()
                        .into_iter()
                        .collect::<Vec<_>>()
                        .join(";"),
                    declaration.line.to_string(),
                    package.owner.clone(),
                    package.phase.clone(),
                    status.into(),
                    evidence.into(),
                    notes.into(),
                    unit,
                    source.hash.clone(),
                    provenance,
                ];
                output
                    .capabilities
                    .push_str(&format!("{}\n", fields.join("\t")));
                *output.counts.entry(package.phase.clone()).or_default() += 1;
            }

            output.unit_rows(package, &entries);
        }

        output.units = format_units(&output.units);
        Ok(output)
    }

    /// Exact regeneration rejects missing, duplicate, altered and invented claims.
    pub fn check(&self, directory: &Path) -> Result<()> {
        compare(
            &fs::read_to_string(directory.join("capabilities.tsv"))?,
            &self.capabilities,
            "capability",
        )?;
        compare(
            &fs::read_to_string(directory.join("source-files.tsv"))?,
            &self.sources,
            "source file",
        )?;

        if fs::read_to_string(directory.join("delivery-units.md"))? != self.units {
            return Err("delivery units differ from pinned extraction".into());
        }

        Ok(())
    }

    /// Writes only extractor-owned files; behavioral evidence is never fabricated.
    pub fn write(&self, directory: &Path) -> Result<()> {
        fs::create_dir_all(directory)?;
        fs::write(directory.join("capabilities.tsv"), &self.capabilities)?;
        fs::write(directory.join("source-files.tsv"), &self.sources)?;
        fs::write(directory.join("delivery-units.md"), &self.units)?;
        Ok(())
    }

    /// Applies corruption controls to the full extracted worklist in memory.
    pub fn controls(&self) -> Result<()> {
        row_controls(&self.capabilities, "capability")?;
        row_controls(&self.sources, "source file")?;
        let fake =
            self.capabilities
                .replacen("\tpending\t-\t", "\tverified\tmissing-evidence.txt\t", 1);

        if compare(&fake, &self.capabilities, "capability").is_ok() {
            return Err("an invented verified/evidence claim was accepted".into());
        }

        println!(
            "controls: missing/duplicate capability and source rows rejected; invented evidence rejected"
        );
        Ok(())
    }

    fn unit_rows(
        &mut self,
        package: &crate::model::Package,
        entries: &BTreeMap<String, (&Source, &crate::model::Declaration)>,
    ) {
        let rows: Vec<_> = entries.iter().collect();

        for (index, chunk) in rows.chunks(UNIT_CAPACITY).enumerate() {
            let first = &chunk[0].0;
            let last = &chunk[chunk.len() - 1].0;
            let source_lines: usize = chunk
                .iter()
                .map(|(_, (source, _))| (source.path.clone(), source.lines))
                .collect::<BTreeMap<_, _>>()
                .values()
                .sum();

            let dependency = match package.phase.as_str() {
                "2" => "core pilot; codec harness",
                "3" => "phase 2 codecs",
                "4" => "phase 2 JSON types; host transport",
                "5" => "phase 3 lifetimes; phase 4 transport",
                "6" => "phase 2 codecs; generator",
                "7" => "phase 3 signers; phase 4/5 host bridge",
                _ => "phase 2-5 typed clients",
            };

            self.units.push_str(&format!(
                "| {}-{:03} | {} | {} | {} | {} | `{}` | `{}` |\n",
                package.name,
                index + 1,
                package.phase,
                chunk.len(),
                source_lines,
                dependency,
                first,
                last
            ));
        }
    }
}

/// Pads the generated Markdown table to the repository formatter's layout.
fn format_units(document: &str) -> String {
    let lines: Vec<_> = document.lines().collect();
    let rows: Vec<Vec<_>> = lines
        .iter()
        .filter(|line| line.starts_with('|'))
        .map(|line| line.trim_matches('|').split('|').map(str::trim).collect())
        .collect();
    let widths: Vec<_> = (0..rows[0].len())
        .map(|column| rows.iter().map(|row| row[column].len()).max().unwrap_or(3))
        .collect();
    let mut result = String::new();
    let mut row_index = 0;

    for line in lines {
        if !line.starts_with('|') {
            result.push_str(line);
            result.push('\n');
            continue;
        }

        result.push('|');

        for (cell, width) in rows[row_index].iter().zip(&widths) {
            let value = if row_index == 1 {
                "-".repeat(*width)
            } else {
                format!("{cell:width$}")
            };
            result.push_str(&format!(" {value} |"));
        }

        result.push('\n');
        row_index += 1;
    }

    result
}

/// Exercise missing and duplicate entries against the actual extracted tables.
fn row_controls(expected: &str, kind: &str) -> Result<()> {
    let rows: Vec<_> = expected.lines().collect();
    let first = rows
        .get(1)
        .ok_or("control requires an extracted source row")?;
    let removed = format!("{}\n{}\n", rows[0], rows[2..].join("\n"));
    let duplicate = format!("{expected}{first}\n");

    for corrupted in [&removed, &duplicate] {
        if compare(corrupted, expected, kind).is_ok() {
            return Err(format!("a corrupted {kind} ledger was accepted").into());
        }
    }

    Ok(())
}

/// Coalesces repeated declarations of the same symbol, such as accessors.
fn collect_entries<'a>(
    source: &'a Source,
    entries: &mut BTreeMap<String, (&'a Source, &'a crate::model::Declaration)>,
) {
    for declaration in &source.exported {
        entries
            .entry(source.id(declaration))
            .or_insert((source, declaration));
    }
}

/// The sole partial claim requires the retained successful native test artifact.
pub(crate) fn validate_pilot(repository: &Path) -> Result<()> {
    let path = repository.join("verification/conformance/evidence/native-success.txt");
    let evidence = fs::read_to_string(path)?;

    if !evidence
        .lines()
        .any(|line| line == "account-role-merge-matrix: Ok(())")
    {
        return Err("partial role-merge evidence is absent".into());
    }

    Ok(())
}

/// Partial pilot evidence cannot complete any broad reference API automatically.
fn pilot(package: &str, symbol: &str) -> (&'static str, &'static str, &'static str) {
    if package == "solana_kit_instructions" && symbol == "mergeRoles" {
        return (
            "partial",
            "verification/conformance/evidence/native-success.txt",
            "AccountRole::merge executes all 16 role combinations; native pilot only; full reference contract review pending",
        );
    }

    (
        "pending",
        "-",
        "source scope only; behavioral evidence required",
    )
}

fn file_row(source: &Source) -> String {
    let warnings = source.parsed.warnings.join(";");
    let coverage = if warnings.is_empty() {
        "parsed-no-unhandled-nodes"
    } else {
        "unresolved"
    };

    let roots = source.roots.iter().cloned().collect::<Vec<_>>().join(";");
    let fields = [
        source.package.clone(),
        source.path.clone(),
        source.hash.clone(),
        source.lines.to_string(),
        source.generated.to_string(),
        source.parsed.declarations.len().to_string(),
        source.exported.len().to_string(),
        roots,
        coverage.into(),
        warnings,
        format!("{PIN}:{}", source.path),
    ];
    format!("{}\n", fields.join("\t"))
}

/// Duplicate IDs get a targeted failure even when a table otherwise looks valid.
pub(crate) fn compare(actual: &str, expected: &str, kind: &str) -> Result<()> {
    let mut ids = std::collections::BTreeSet::new();

    for row in actual.lines().skip(1) {
        let column = usize::from(kind == "source file");
        let id = row.split('\t').nth(column).ok_or("empty ledger row")?;

        if !ids.insert(id) {
            return Err(format!("duplicate {kind} row: {id}").into());
        }
    }

    if actual != expected {
        return Err(format!("{kind} ledger differs from pinned extraction; missing, altered or unsupported evidence row").into());
    }

    Ok(())
}

fn units_header() -> String {
    "# Remaining implementation units\n\nThese units build an off-chain client SDK for applications. Program clients construct\nand decode program interactions; they do not make this SDK an on-chain program\nlibrary. Use Pina or official Solana program libraries for on-chain code.\n\nGenerated from the capability ledger. Each unit has at most 200 exact IDs; the `unit`\ncolumn in capabilities.tsv is its authoritative membership, including both range\nendpoints below. Units are source-order planning tranches, not promises that a\n200-symbol change is one reviewable PR. Split at source/type boundaries during\nimplementation and retain ID assignments. Source lines count distinct defining\nfiles touched by the tranche, not implementation effort. All units remain pending.\n\nShared codec composition and a reproducible Kit-owned generator should precede\nprogram tranches. Generated clients share layouts, discriminator rules, builders,\nand negative fixtures; do not manually port repeated output before this leverage\nis proved. Consult core-codecs.tsv and program-inputs.tsv for their independent\naudit results. Mobile units depend on an installed-host bridge, user gestures,\nwallet availability, cancellation/unload, emulator and simulator evidence; the\nreference platform conditionals do not prove Rust host support.\n\n| Unit | Phase | IDs | Source lines | Dependencies | First exact ID | Last exact ID |\n| --- | --- | --- | --- | --- | --- | --- |\n".into()
}
