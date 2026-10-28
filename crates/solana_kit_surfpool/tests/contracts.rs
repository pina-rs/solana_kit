//! Every assertion body is also registered in the integrated portable suite.

mod support;

#[test]
fn public_surfpool_contracts() {
    for (name, contract) in support::groups() {
        assert_eq!(contract(), Ok(()), "{name}");
    }
}
