use std::{env, error::Error, fs, path::Path};

type Result<Value> = std::result::Result<Value, Box<dyn Error>>;

fn main() -> Result<()> {
    let arguments: Vec<_> = env::args().collect();
    let reference = Path::new(arguments.get(1).ok_or("missing reference")?);
    let repository = Path::new(arguments.get(2).ok_or("missing repository")?);
    let source = fs::read_to_string(
        reference.join("packages/solana_kit_wallet_standard/lib/src/logos.dart"),
    )?;
    let assets = repository.join("crates/solana_kit_wallet/assets");
    fs::create_dir_all(&assets)?;
    let mut table = String::from("pub(crate) const LOGOS: &[(&str, &str)] = &[\n");
    let mut count = 0;

    for line in source.lines() {
        count += extract(line.trim(), &assets, &mut table)?;
    }

    table.push_str("];\n");
    fs::write(
        repository.join("crates/solana_kit_wallet/src/logo_assets.rs"),
        table,
    )?;
    println!("Extracted {count} exact pinned fallback data URIs and generic icon");
    Ok(())
}

fn extract(line: &str, assets: &Path, table: &mut String) -> Result<usize> {
    let Some(literal) = line.strip_prefix('\'') else {
        return Ok(0);
    };

    if let Some((name, uri)) = literal.split_once("': '") {
        let uri = uri.strip_suffix("',").ok_or("bad logo literal")?;
        fs::write(assets.join(format!("{name}.uri")), uri)?;
        table.push_str(&format!(
            "    ({name:?}, include_str!(\"../assets/{name}.uri\")),\n"
        ));

        return Ok(1);
    }

    if let Some(uri) = literal.strip_suffix("';") {
        fs::write(assets.join("generic.uri"), uri)?;
    }

    Ok(0)
}
