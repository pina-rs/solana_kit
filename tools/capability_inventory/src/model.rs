//! Source identities and records shared by extraction and validation.

use std::collections::BTreeSet;

pub(crate) const PIN: &str = "0138bb956d2d81ccc0c56edc602401c0df2f2b46";
pub(crate) type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Debug)]
pub(crate) struct Package {
    pub name: String,
    pub manifest: String,
    pub owner: String,
    pub phase: String,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct Declaration {
    pub symbol: String,
    pub root: String,
    pub kind: String,
    pub line: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct Export {
    pub target: String,
    pub show: BTreeSet<String>,
    pub hide: BTreeSet<String>,
}

impl Export {
    /// Applies Dart's namespace combinators to the containing top-level name.
    pub fn includes(&self, name: &str) -> bool {
        (self.show.is_empty() || self.show.contains(name)) && !self.hide.contains(name)
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Parsed {
    pub declarations: Vec<Declaration>,
    pub exports: Vec<Export>,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug)]
pub(crate) struct Source {
    pub package: String,
    pub path: String,
    pub hash: String,
    pub lines: usize,
    pub generated: bool,
    pub parsed: Parsed,
    pub exported: BTreeSet<Declaration>,
    pub roots: BTreeSet<String>,
}

impl Source {
    /// Uses the defining package and symbol rather than a mutable source line.
    pub fn id(&self, declaration: &Declaration) -> String {
        let language = if self.path.ends_with(".ts") {
            "ts"
        } else {
            "dart"
        };

        let relative = self.path.splitn(3, '/').nth(2).unwrap_or(&self.path);
        format!(
            "{language}:{}:{relative}#{}",
            self.package, declaration.symbol
        )
    }
}
