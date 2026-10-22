#[test]
fn isolated_compute_budget_cases_execute_natively() {
    solana_kit_conformance::compute_budget::full_width_price().unwrap();
    solana_kit_conformance::compute_budget::ordering_and_historical_layout().unwrap();
}
