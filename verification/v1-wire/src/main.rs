//! Loopback-only validator probe using public deterministic fixture keys.

use std::error::Error;
use std::process::Command;

use base64::{Engine, engine::general_purpose::STANDARD};
use serde_json::{Value, json};
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Hash, Instruction,
    crypto::Keypair,
    transaction::{Lifetime, Message, Transaction, V1_PACKET_SIZE as MAX_PACKET, V1Config},
};

const STATUS_POLL_MS: u64 = 200;
const COMPUTE_LIMIT: u32 = 200_000;
const LOADED_LIMIT: u32 = 1_048_576;
const LARGE_PADDING: usize = 1600;
const SIZE_ERROR: i64 = -32602;
#[cfg(test)]
const INTERNAL_ERROR: i64 = -32603;

fn rpc(method: &str, params: Value) -> Result<Value, Box<dyn Error>> {
    let body = json!({"jsonrpc":"2.0", "id":1, "method":method, "params":params});
    let output = Command::new("curl")
        .args([
            "--silent",
            "--show-error",
            "--max-time",
            "10",
            "-H",
            "Content-Type: application/json",
            "--data",
        ])
        .arg(body.to_string())
        .arg("http://127.0.0.1:19899")
        .output()?;

    if !output.status.success() {
        return Err(format!("local RPC process failed: {}", output.status).into());
    }

    Ok(serde_json::from_slice(&output.stdout)?)
}

fn signature_wire(message: &Message, key: &Keypair) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(Transaction::new(message.clone()).sign(key)?.encode(true)?)
}

fn simulate(name: &str, wire: &[u8]) -> Result<Value, Box<dyn Error>> {
    let response = rpc(
        "simulateTransaction",
        json!([STANDARD.encode(wire), {
            "encoding":"base64", "sigVerify":true, "commitment":"confirmed"
        }]),
    )?;
    println!("{name} bytes={} response={response}", wire.len());
    Ok(response)
}

fn control_wire(payer: &[u8; 32], hash: &[u8; 32], key: &Keypair) -> Vec<u8> {
    let mut message = vec![1, 0, 1, 3];
    message.extend(payer);
    message.extend([8; 32]);
    message.extend([0; 32]);
    message.extend(hash);
    message.extend([1, 2, 2, 0, 1, 12]);
    message.extend(2u32.to_le_bytes());
    message.extend(10_000_000u64.to_le_bytes());

    let mut wire = vec![1];
    wire.extend(key.sign(&message).as_bytes());
    wire.extend(message);
    wire
}

fn base_message(
    payer: Address,
    hash: Hash,
    padding: Option<usize>,
) -> Result<Message, Box<dyn Error>> {
    let mut data = 2u32.to_le_bytes().to_vec();
    data.extend(10_000_000u64.to_le_bytes());
    let mut instructions = vec![Instruction {
        program_address: Address::from_bytes([0; 32]),
        accounts: vec![
            AccountMeta {
                address: payer,
                role: AccountRole::WritableSigner,
            },
            AccountMeta {
                address: Address::from_bytes([8; 32]),
                role: AccountRole::Writable,
            },
        ],
        data,
    }];

    if let Some(count) = padding {
        instructions.push(Instruction {
            program_address: "ComputeBudget111111111111111111111111111111".parse()?,
            accounts: vec![],
            data: vec![7; count],
        });
    }

    Ok(Message::compile_v1(
        payer,
        Lifetime::Blockhash {
            blockhash: hash,
            last_valid_block_height: u64::MAX,
        },
        &instructions,
        V1Config::default()
            .with_compute_unit_limit(COMPUTE_LIMIT)
            .with_loaded_accounts_data_limit(LOADED_LIMIT),
    )?)
}

/// Polls local fixture submissions with matching commitment to avoid hash ambiguity.
fn send_and_confirm(name: &str, wire: &[u8]) -> Result<(), Box<dyn Error>> {
    let response = rpc(
        "sendTransaction",
        json!([STANDARD.encode(wire), {"encoding":"base64", "preflightCommitment":"confirmed"}]),
    )?;
    println!("{name} send={response}");
    let signature = response["result"].as_str().ok_or("submission rejected")?;

    for _ in 0..20 {
        let status = rpc("getSignatureStatuses", json!([[signature]]))?;
        let value = &status["result"]["value"][0];

        if !value.is_null() && value["confirmationStatus"].as_str() != Some("processed") {
            println!("{name} status={status}");

            if !value["err"].is_null() {
                return Err("confirmed transaction execution failed".into());
            }

            return readback(name, signature);
        }

        std::thread::sleep(std::time::Duration::from_millis(STATUS_POLL_MS));
    }

    Err("confirmation deadline".into())
}

fn readback(name: &str, signature: &str) -> Result<(), Box<dyn Error>> {
    let fetched = rpc(
        "getTransaction",
        json!([signature, {"encoding":"json", "commitment":"confirmed", "maxSupportedTransactionVersion":1}]),
    )?;
    let expected = if name == "legacy-control" {
        json!("legacy")
    } else {
        json!(1)
    };

    if fetched["result"]["version"] != expected || !fetched["result"]["meta"]["err"].is_null() {
        return Err("readback version or execution status mismatch".into());
    }

    println!("{name} fetched={fetched}");
    Ok(())
}

/// Requires an executed legacy control before interpreting v1 feature-gate results.
fn main() -> Result<(), Box<dyn Error>> {
    let key = Keypair::from_seed(&std::array::from_fn(|index| {
        u8::try_from(index + 1).unwrap()
    }));
    let version = rpc("getVersion", json!([]))?;
    let feature = rpc(
        "getAccountInfo",
        json!(["txv1aq4pp281K9um3tnPgkfX8UqtFT6wcVW3hNezGLL", {"encoding":"base64"}]),
    )?;
    println!(
        "version={version}\nv1-feature={feature}\npayer={}",
        key.address()
    );
    let latest = rpc("getLatestBlockhash", json!([{"commitment":"confirmed"}]))?;
    let hash: Address = latest["result"]["value"]["blockhash"]
        .as_str()
        .ok_or("missing blockhash")?
        .parse()?;
    legacy_control(&key, &hash)?;
    let message = base_message(key.address(), Hash::from_bytes(*hash.as_bytes()), None)?;
    let small = signature_wire(&message, &key)?;

    let small_result = simulate("v1-small", &small)?;

    if std::env::args().nth(1).as_deref() == Some("--expect-disabled") {
        let error = small_result["result"]["value"]["err"].as_str();

        if error != Some("UnsupportedVersion") {
            return Err("disabled feature did not report UnsupportedVersion".into());
        }

        return Ok(());
    }

    if small_result.get("error").is_some() || !small_result["result"]["value"]["err"].is_null() {
        return Err("v1 support probe rejected; see exact response".into());
    }

    send_and_confirm("v1-small", &small)?;
    signature_negative(&small)?;
    large_probe(Hash::from_bytes(*hash.as_bytes()), &key)?;
    Ok(())
}

fn legacy_control(key: &Keypair, hash: &Address) -> Result<(), Box<dyn Error>> {
    let control = control_wire(key.address().as_bytes(), hash.as_bytes(), key);
    let control_result = simulate("legacy-control", &control)?;

    if !control_result["result"]["value"]["err"].is_null() || control_result.get("error").is_some()
    {
        return Err("legacy control failed".into());
    }

    send_and_confirm("legacy-control", &control)
}

fn large_probe(hash: Hash, key: &Keypair) -> Result<(), Box<dyn Error>> {
    let message = base_message(key.address(), hash, Some(LARGE_PADDING))?;
    let wire = signature_wire(&message, key)?;
    let result = simulate("v1-large-noop", &wire)?;

    if result.get("error").is_some() || !result["result"]["value"]["err"].is_null() {
        return Err("large v1 execution failed".into());
    }

    send_and_confirm("v1-large-noop", &wire)?;
    let padding = LARGE_PADDING + MAX_PACKET - wire.len();
    let message = base_message(key.address(), hash, Some(padding))?;
    let exact = signature_wire(&message, key)?;
    assert_eq!(exact.len(), MAX_PACKET);
    let exact_result = simulate("v1-exact-cap", &exact)?;

    if exact_result.get("error").is_some() || !exact_result["result"]["value"]["err"].is_null() {
        return Err("4096-byte v1 rejected".into());
    }

    send_and_confirm("v1-exact-cap", &exact)?;
    assert!(base_message(key.address(), hash, Some(padding + 1)).is_err());
    let mut over = exact;
    over.push(0);
    let rejected = simulate("v1-over-cap", &over)?;

    if !is_size_error(&rejected) {
        return Err("4097 bytes unexpectedly accepted at RPC boundary".into());
    }

    Ok(())
}

fn signature_negative(wire: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut corrupt = wire.to_vec();
    let last = corrupt.last_mut().ok_or("empty wire")?;
    *last ^= 1;
    let response = simulate("v1-corrupt-signature", &corrupt)?;

    if response["result"]["value"]["err"].as_str() != Some("SignatureFailure") {
        return Err("corrupted signature was not rejected".into());
    }

    Ok(())
}

fn is_size_error(response: &Value) -> bool {
    let error = &response["error"];
    let message = error["message"].as_str().unwrap_or("");
    error["code"].as_i64() == Some(SIZE_ERROR)
        && message.contains("VersionedTransaction too large")
        && message.contains("4097")
        && message.contains("4096")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_cap_requires_the_specific_node_size_error() {
        let exact = json!({"error":{"code":SIZE_ERROR,"message":"decoded VersionedTransaction too large: 4097 bytes (max: 4096 bytes)"}});
        assert!(is_size_error(&exact));

        for response in [
            json!({"error":{"code":INTERNAL_ERROR,"message":"internal error"}}),
            json!({"error":{"code":SIZE_ERROR,"message":"invalid base64"}}),
            json!({"result":null}),
            json!({"error":{"code":SIZE_ERROR,"message":"VersionedTransaction too large: 5000 (max: 1232)"}}),
        ] {
            assert!(!is_size_error(&response));
        }
    }
}
