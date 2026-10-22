//! Literal public Dart role expectations over heterogeneous Rust backend types.

use super::*;

const PARTIAL_SENDING: u8 = 5;
const MODIFYING_SENDING: u8 = 6;
const COMPOSITE: u8 = 7;
const INTERFACE_COUNT: usize = 7;
const SIXTH: usize = 5;
const SEVENTH: usize = 6;
const PAIR_CASES: usize = 112;
const TRIPLE_CASES: usize = 698;

struct Backend<const TAG: u8>(Rc<Cell<usize>>);

struct Backends {
    first: Backend<0>,
    second: Backend<1>,
    third: Backend<2>,
    fourth: Backend<3>,
    fifth: Backend<4>,
    sixth: Backend<5>,
    seventh: Backend<6>,
}

fn backends(effects: &Rc<Cell<usize>>) -> Backends {
    Backends {
        first: Backend(effects.clone()),
        second: Backend(effects.clone()),
        third: Backend(effects.clone()),
        fourth: Backend(effects.clone()),
        fifth: Backend(effects.clone()),
        sixth: Backend(effects.clone()),
        seventh: Backend(effects.clone()),
    }
}

impl<const TAG: u8> Signer for Backend<TAG> {
    fn address(&self) -> Address {
        Address::from_bytes([TAG; 32])
    }
}

impl<const TAG: u8> TransactionPartialSigner for Backend<TAG> {
    type Sign<'a> = Ready<Result<SignatureOutput, Error>>;
    fn sign_transaction<'a>(&'a self, _: &'a Transaction) -> Self::Sign<'a> {
        self.0.set(self.0.get() + 1);
        ready(Err(Error::Rejected))
    }
}

impl<const TAG: u8> TransactionModifyingSigner for Backend<TAG> {
    type Modify<'a> = Ready<Result<Transaction, Error>>;
    fn modify_transaction<'a>(&'a self, tx: &'a Transaction) -> Self::Modify<'a> {
        self.0.set(self.0.get() + 1);
        ready(Ok(tx.clone()))
    }
}

impl<const TAG: u8> TransactionSendingSigner for Backend<TAG> {
    type Send<'a> = Ready<Result<Signature, Error>>;
    fn send_transaction<'a>(&'a self, _: &'a Transaction) -> Self::Send<'a> {
        self.0.set(self.0.get() + 1);
        ready(Ok(Signature::from_bytes([TAG; 64])))
    }
}

fn declared<'a, const TAG: u8>(
    flags: u8,
    handle: &'a SignerCapability<'a, Backend<TAG>>,
) -> Result<Registration<'a>, &'static str> {
    match flags {
        1 => Ok(Registration::partial(handle)),
        2 => Ok(Registration::modifying(handle)),
        3 => Ok(Registration::partial_modifying(handle)),
        4 => Ok(Registration::sending(handle)),
        PARTIAL_SENDING => Ok(Registration::partial_sending(handle)),
        MODIFYING_SENDING => Ok(Registration::modifying_sending(handle)),
        COMPOSITE => Ok(Registration::composite(handle)),
        _ => Err("invalid literal interface flags"),
    }
}

fn render(
    result: Result<Vec<(Address, Role)>, Error>,
) -> Result<alloc::string::String, &'static str> {
    match result {
        Ok(roles) => Ok(roles
            .into_iter()
            .map(|(_, role)| match role {
                Role::Partial => "p",
                Role::Modifying => "m",
                Role::Sending => "s",
                Role::Ignored => "i",
            })
            .collect::<Vec<_>>()
            .join(",")),
        Err(Error::MissingSender) => Ok("missing".into()),
        Err(Error::MultipleSenders) => Ok("multiple".into()),
        Err(_) => Err("unexpected automatic role error"),
    }
}

fn literal(line: &str) -> Outcome {
    let columns = line.split('\t').collect::<Vec<_>>();
    crate::require(columns.len() == 3, "literal role column count")?;
    let mut flags = columns[0]
        .split(',')
        .map(str::parse::<u8>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "literal role flag")?;
    let count = flags.len();
    flags.resize(INTERFACE_COUNT, 1);

    compare(&flags, count, columns[1] == "1", columns[2])
}

fn compare(flags: &[u8], count: usize, sending: bool, expected: &str) -> Outcome {
    let effects = Rc::new(Cell::new(0));
    let backends = backends(&effects);
    let (
        first_handle,
        second_handle,
        third_handle,
        fourth_handle,
        fifth_handle,
        sixth_handle,
        seventh_handle,
    ) = (
        SignerCapability::new(&backends.first),
        SignerCapability::new(&backends.second),
        SignerCapability::new(&backends.third),
        SignerCapability::new(&backends.fourth),
        SignerCapability::new(&backends.fifth),
        SignerCapability::new(&backends.sixth),
        SignerCapability::new(&backends.seventh),
    );

    let mut registrations = vec![
        declared(flags[0], &first_handle)?,
        declared(flags[1], &second_handle)?,
        declared(flags[2], &third_handle)?,
        declared(flags[3], &fourth_handle)?,
        declared(flags[4], &fifth_handle)?,
        declared(flags[SIXTH], &sixth_handle)?,
        declared(flags[SEVENTH], &seventh_handle)?,
    ];
    registrations.truncate(count);
    let composition = Composition::new(registrations).map_err(|_| "literal composition")?;

    crate::require(
        render(composition.roles(sending))? == expected,
        "literal Dart automatic role mismatch",
    )?;
    crate::require(
        effects.get() == 0,
        "role inspection must precede all backend effects",
    )
}

pub(super) fn run() -> Outcome {
    let pairs = include_str!("../../../signer-capabilities/fixtures/dart-roles.tsv");
    let triples = include_str!("../../fixtures/signer-triple-roles.tsv");
    crate::require(
        pairs.lines().count() == PAIR_CASES && triples.lines().count() == TRIPLE_CASES,
        "literal oracle cardinality",
    )?;

    for line in pairs.lines().chain(triples.lines()) {
        literal(line)?;
    }

    Ok(())
}
