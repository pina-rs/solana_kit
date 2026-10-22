use solana_compute_budget_interface::ComputeBudgetInstruction as Official;
use solana_kit_compute_budget::*;
use solana_kit_core::codecs::Limits;

const MAX_INSTRUCTION_BYTES: usize = 9;
const MIXED_ENDIAN_VALUE: u32 = 257;

#[test]
fn official_current_constructors_match_bytes_program_and_empty_metas() {
    for amount in [0, 1, MIXED_ENDIAN_VALUE, u32::MAX] {
        let pairs = [
            (
                RequestHeapFrame { bytes: amount }
                    .instruction(Limits::new(MAX_INSTRUCTION_BYTES, 0))
                    .unwrap(),
                Official::request_heap_frame(amount),
            ),
            (
                SetComputeUnitLimit { units: amount }
                    .instruction(Limits::new(MAX_INSTRUCTION_BYTES, 0))
                    .unwrap(),
                Official::set_compute_unit_limit(amount),
            ),
            (
                SetLoadedAccountsDataSizeLimit {
                    account_data_size_limit: amount,
                }
                .instruction(Limits::new(MAX_INSTRUCTION_BYTES, 0))
                .unwrap(),
                Official::set_loaded_accounts_data_size_limit(amount),
            ),
        ];

        for (kit, official) in pairs {
            assert_eq!(kit.data, official.data);
            assert_eq!(
                kit.program_address.as_bytes(),
                &official.program_id.to_bytes()
            );
            assert!(kit.accounts.is_empty() && official.accounts.is_empty());
        }
    }

    for amount in [0, 1, u32::MAX.into(), u64::MAX] {
        let kit = SetComputeUnitPrice {
            micro_lamports: amount,
        }
        .instruction(Limits::new(MAX_INSTRUCTION_BYTES, 0))
        .unwrap();
        let official = Official::set_compute_unit_price(amount);
        assert_eq!(kit.data, official.data);
        assert_eq!(
            kit.program_address.as_bytes(),
            &official.program_id.to_bytes()
        );
        assert!(kit.accounts.is_empty() && official.accounts.is_empty());
    }
}
