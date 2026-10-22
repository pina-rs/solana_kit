//! Loopback-only validator probe using public deterministic fixture keys.

use std::error::Error;
use std::process::Command;

use base64::{Engine, engine::general_purpose::STANDARD};
use serde_json::{Value, json};
use solana_kit_core::{Address, crypto::Keypair};
use solana_kit_v1_wire_proof::{Instruction, MAX_PACKET, Message, encode};

const STATUS_POLL_MS: u64 = 200;

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
        .arg("http://127.0.0.1:18899")
        .output()?;

    if !output.status.success() {
        return Err(format!("local RPC process failed: {}", output.status).into());
    }

    Ok(serde_json::from_slice(&output.stdout)?)
}

fn signature_wire(message: &Message, key: &Keypair) -> Result<Vec<u8>, Box<dyn Error>> {
    let empty = encode(message, &[[0; 64]])?;
    let signature = key.sign(&empty[..empty.len() - 64]);
    Ok(encode(message, &[*signature.as_bytes()])?)
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

fn base_message(payer: [u8; 32], hash: [u8; 32]) -> Message {
    let mut data = 2u32.to_le_bytes().to_vec();
    data.extend(10_000_000u64.to_le_bytes());
    let mut config = 200_000u32.to_le_bytes().to_vec();
    config.extend(1_048_576u32.to_le_bytes());
    Message {
        header: [1, 0, 1],
        mask: 12,
        lifetime: hash,
        addresses: vec![payer, [8; 32], [0; 32]],
        config,
        instructions: vec![Instruction {
            program: 2,
            accounts: vec![0, 1],
            data,
        }],
    }
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
    let message = base_message(*key.address().as_bytes(), *hash.as_bytes());
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
    large_probe(message, &key)?;
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

fn large_probe(mut message: Message, key: &Keypair) -> Result<(), Box<dyn Error>> {
    let budget: Address = "ComputeBudget111111111111111111111111111111".parse()?;
    message.addresses.push(*budget.as_bytes());
    message.header[2] += 1;
    message.instructions.push(Instruction {
        program: 3,
        accounts: vec![],
        data: vec![7; 1600],
    });
    let wire = signature_wire(&message, key)?;
    let result = simulate("v1-large-noop", &wire)?;

    if result.get("error").is_some() || !result["result"]["value"]["err"].is_null() {
        return Err("large v1 execution failed".into());
    }

    send_and_confirm("v1-large-noop", &wire)?;
    let increase = MAX_PACKET - wire.len();
    message.instructions[1].data.extend(vec![7; increase]);
    let exact = signature_wire(&message, key)?;
    assert_eq!(exact.len(), MAX_PACKET);
    let exact_result = simulate("v1-exact-cap", &exact)?;

    if exact_result.get("error").is_some() || !exact_result["result"]["value"]["err"].is_null() {
        return Err("4096-byte v1 rejected".into());
    }

    send_and_confirm("v1-exact-cap", &exact)?;
    let mut over = exact;
    over.push(0);
    let rejected = simulate("v1-over-cap", &over)?;

    if rejected.get("error").is_none() {
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
