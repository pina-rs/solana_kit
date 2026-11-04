//! Executable proof for the pinned `SolanaErrorDomain` classification.
//!
//! The fixture transcribes every `getSolanaErrorDomain` range from the pinned
//! source. Probes cover each interval endpoint, the gaps between intervals, the
//! historical account typo and integers the Dart enum cannot represent. The same
//! fixture is cross-checked against the independent per-code domain column.

use solana_kit_core::errors::{Context, Diagnostic, ErrorCode, ErrorDomain};
use std::{borrow::ToOwned, collections::BTreeSet, string::String, vec::Vec};

/// Number of `SolanaErrorDomain` values pinned by the source.
pub const DOMAINS: usize = 25;

const ROW_FIELDS: usize = 3;
const CATALOG_FIELDS: usize = 4;

/// The first failed comparison, named for native and portable diagnostics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mismatch {
    /// Domain or `<domains>` label.
    pub name: String,
    /// Stable explanation of the first failed comparison.
    pub field: &'static str,
}

impl Mismatch {
    fn new(name: impl Into<String>, field: &'static str) -> Self {
        Self {
            name: name.into(),
            field,
        }
    }
}

/// One pinned domain and its inclusive source ranges.
struct Domain {
    name: String,
    range: Vec<(i64, i64)>,
}

/// Compares every domain variant, every range boundary and every pinned code.
pub fn exact(domains: &str, catalog: &str) -> Result<(), Mismatch> {
    let domains = parse_domains(domains)?;
    let values = catalog_values(catalog)?;
    check_variants(&domains)?;
    check_catalog_domains(&domains, catalog)?;

    for probe in probes(&domains, &values) {
        check_probe(&domains, &values, probe)?;
    }

    check_membership_helpers(&domains)
}

/// Splits fixture rows without accepting missing or extra columns.
fn parse_domains(fixture: &str) -> Result<Vec<Domain>, Mismatch> {
    let mut domains = Vec::new();

    for line in fixture.lines() {
        let fields: Vec<_> = line.split('\t').collect();

        if fields.len() != ROW_FIELDS {
            return Err(Mismatch::new(
                "<domains>",
                "domain row does not have three fields",
            ));
        }

        let mut range = Vec::new();

        if fields[2] != "-" {
            for interval in fields[2].split(';') {
                let (minimum, maximum) = interval.split_once("..").ok_or(Mismatch::new(
                    fields[1],
                    "domain range is not an inclusive interval",
                ))?;
                let minimum: i64 = minimum
                    .parse()
                    .map_err(|_| Mismatch::new(fields[1], "domain minimum is not an integer"))?;
                let maximum: i64 = maximum
                    .parse()
                    .map_err(|_| Mismatch::new(fields[1], "domain maximum is not an integer"))?;

                if minimum > maximum {
                    return Err(Mismatch::new(fields[1], "domain range is inverted"));
                }

                range.push((minimum, maximum));
            }
        }

        domains.push(Domain {
            name: fields[0].to_owned(),
            range,
        });
    }

    Ok(domains)
}

/// Reads the independent per-code values and domains.
fn catalog_values(catalog: &str) -> Result<Vec<(i64, String)>, Mismatch> {
    let mut values = Vec::new();

    for line in catalog.lines() {
        let fields: Vec<_> = line.split('\t').collect();

        if fields.len() < CATALOG_FIELDS {
            return Err(Mismatch::new(
                "<domains>",
                "catalog row does not have four fields",
            ));
        }

        let value: i64 = fields[1]
            .parse()
            .map_err(|_| Mismatch::new("<domains>", "catalog code is not an integer"))?;
        values.push((value, fields[2].to_owned()));
    }

    Ok(values)
}

/// Requires all 25 distinct Rust variants in pinned source order.
fn check_variants(domains: &[Domain]) -> Result<(), Mismatch> {
    if domains.len() != DOMAINS {
        return Err(Mismatch::new(
            "<domains>",
            "domain fixture does not have 25 rows",
        ));
    }

    let mut names = BTreeSet::new();

    for domain in domains {
        if !names.insert(domain.name.as_str()) {
            return Err(Mismatch::new(
                domain.name.clone(),
                "domain fixture is duplicated",
            ));
        }
    }

    Ok(())
}

/// Every pinned code in the catalog must classify as its independent domain.
fn check_catalog_domains(domains: &[Domain], catalog: &str) -> Result<(), Mismatch> {
    for line in catalog.lines() {
        let fields: Vec<_> = line.split('\t').collect();
        let value: i64 = fields[1]
            .parse()
            .map_err(|_| Mismatch::new("<domains>", "catalog code is not an integer"))?;
        let expected = classify(domains, value)?;
        let actual = classify_rust(value);

        if actual != expected {
            return Err(Mismatch::new(
                expected,
                "catalog code does not classify as its pinned domain",
            ));
        }

        if fields[2] != expected {
            return Err(Mismatch::new(
                expected,
                "fixture range disagrees with the catalog domain",
            ));
        }
    }

    Ok(())
}

/// Returns the domain whose source interval contains the value.
fn classify(domains: &[Domain], value: i64) -> Result<String, Mismatch> {
    for domain in domains {
        if domain
            .range
            .iter()
            .any(|(minimum, maximum)| (*minimum..=*maximum).contains(&value))
        {
            return Ok(domain.name.clone());
        }
    }

    let unknown = domains
        .iter()
        .find(|domain| domain.range.is_empty())
        .ok_or(Mismatch::new("<domains>", "no fallback domain is pinned"))?;

    if unknown.name != "unknown" {
        return Err(Mismatch::new(
            unknown.name.clone(),
            "fallback domain is not the source unknown value",
        ));
    }

    Ok(unknown.name.clone())
}

/// Reads the Rust variant's exact lower-camel source-equivalent name.
fn classify_rust(value: i64) -> String {
    domain_name(ErrorCode::from_value(value).domain()).to_owned()
}

/// Maps the Rust variant to the pinned lower-camel constant name.
fn domain_name(domain: ErrorDomain) -> &'static str {
    match domain {
        ErrorDomain::General => "general",
        ErrorDomain::JsonRpc => "jsonRpc",
        ErrorDomain::Addresses => "addresses",
        ErrorDomain::Accounts => "accounts",
        ErrorDomain::SubtleCrypto => "subtleCrypto",
        ErrorDomain::Crypto => "crypto",
        ErrorDomain::Keys => "keys",
        ErrorDomain::Instruction => "instruction",
        ErrorDomain::InstructionError => "instructionError",
        ErrorDomain::Signer => "signer",
        ErrorDomain::OffchainMessage => "offchainMessage",
        ErrorDomain::Transaction => "transaction",
        ErrorDomain::TransactionError => "transactionError",
        ErrorDomain::TransactionIntrospection => "transactionIntrospection",
        ErrorDomain::InstructionPlans => "instructionPlans",
        ErrorDomain::Codecs => "codecs",
        ErrorDomain::Rpc => "rpc",
        ErrorDomain::RpcSubscriptions => "rpcSubscriptions",
        ErrorDomain::Subscribable => "subscribable",
        ErrorDomain::ProgramClients => "programClients",
        ErrorDomain::MobileWalletAdapter => "mobileWalletAdapter",
        ErrorDomain::Helius => "helius",
        ErrorDomain::Wallet => "wallet",
        ErrorDomain::InvariantViolation => "invariantViolation",
        ErrorDomain::Unknown => "unknown",
    }
}

/// Collects every boundary, gap and unrepresentable integer exactly once.
fn probes(domains: &[Domain], catalog: &[(i64, String)]) -> BTreeSet<i64> {
    let mut probes = BTreeSet::from([i64::MIN, i64::MAX, -1, 0, 1, 11]);

    for domain in domains {
        for (minimum, maximum) in &domain.range {
            probes.insert(*minimum);
            probes.insert(*maximum);
            probes.insert(minimum.saturating_sub(1));
            probes.insert(maximum.saturating_add(1));
            probes.insert(minimum + (maximum - minimum) / 2);
        }
    }

    for (value, _) in catalog {
        probes.insert(*value);
    }

    probes
}

/// Checks one probe against the source ranges and its Rust definition status.
fn check_probe(domains: &[Domain], catalog: &[(i64, String)], probe: i64) -> Result<(), Mismatch> {
    let expected = classify(domains, probe)?;
    let actual = classify_rust(probe);

    if actual != expected {
        return Err(Mismatch::new(
            expected,
            "domain range endpoint differs from the pinned rule",
        ));
    }

    let defined = catalog.iter().any(|(value, _)| *value == probe);
    let code = ErrorCode::from_value(probe);

    if code.definition().is_some() != defined {
        return Err(Mismatch::new(
            expected,
            "unknown integer is confused with a pinned code",
        ));
    }

    if code.value() != probe {
        return Err(Mismatch::new(expected, "probed integer is not preserved"));
    }

    Ok(())
}

/// Exercises the source membership helpers and the diagnostic extension.
fn check_membership_helpers(domains: &[Domain]) -> Result<(), Mismatch> {
    for domain in domains {
        let Some((minimum, _)) = domain.range.first().copied() else {
            continue;
        };
        let code = ErrorCode::from_value(minimum);

        if classify_rust(minimum) != domain.name {
            return Err(Mismatch::new(
                domain.name.clone(),
                "domain helper disagrees with the Rust variant",
            ));
        }

        let other = domains
            .iter()
            .find(|candidate| candidate.name != domain.name && !candidate.range.is_empty())
            .ok_or(Mismatch::new(
                domain.name.clone(),
                "no second domain is pinned",
            ))?;
        let other_value = other.range[0].0;

        if code.domain() == ErrorCode::from_value(other_value).domain() {
            return Err(Mismatch::new(
                other.name.clone(),
                "distinct domains compare equal",
            ));
        }

        let diagnostic = Diagnostic::new(code, Context::default());

        if diagnostic.code().domain() != code.domain() {
            return Err(Mismatch::new(
                domain.name.clone(),
                "diagnostic domain differs from its code",
            ));
        }
    }

    Ok(())
}

/// Proves a one-integer range corruption is rejected for the intended domain.
pub fn rejects_wrong_range(domains: &str, catalog: &str) -> Result<(), Mismatch> {
    exact(domains, catalog)?;

    let corrupted = domains.replacen(
        "addresses\tAddresses\t2800000..2800999",
        "addresses\tAddresses\t2800001..2800999",
        1,
    );

    if corrupted == domains {
        return Err(Mismatch::new(
            "addresses",
            "control could not mutate the pinned range",
        ));
    }

    match exact(&corrupted, catalog) {
        Ok(()) => Err(Mismatch::new(
            "addresses",
            "wrong domain range was accepted",
        )),
        Err(mismatch)
            if mismatch.name == "unknown"
                && mismatch.field == "catalog code does not classify as its pinned domain" =>
        {
            Ok(())
        }
        Err(_) => Err(Mismatch::new(
            "addresses",
            "wrong domain range failed for the wrong reason",
        )),
    }
}
