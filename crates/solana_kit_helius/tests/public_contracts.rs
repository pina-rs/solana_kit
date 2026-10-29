mod contracts;

#[test]
fn maintained_public_contracts() -> Result<(), &'static str> {
    contracts::run_all()
}
