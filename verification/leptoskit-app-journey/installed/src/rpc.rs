//! Independent bounded JSON-RPC observations; state-changing calls execute once.
use crate::{Error, Result, require};
use reqwest::{Url, blocking::Client, redirect::Policy};
use serde_json::{Value, json};
use std::{io::Read, time::Duration};

const REPLY_BYTES: u64 = 64 * 1024;

pub(crate) fn loopback_url(value: &str) -> Result<Url> {
    let url = Url::parse(value).map_err(|error| Error::Configuration(error.to_string()))?;
    let address = url.host_str().and_then(|host| {
        host.trim_matches(['[', ']'])
            .parse::<std::net::IpAddr>()
            .ok()
    });
    require(
        url.scheme() == "http"
            && address.is_some_and(|address| address.is_loopback())
            && url.port().is_some_and(|port| port != 0)
            && url.username().is_empty()
            && url.password().is_none()
            && url.query().is_none()
            && url.fragment().is_none()
            && url.path() == "/",
        "an explicit numeric loopback HTTP authority is required",
    )?;
    Ok(url)
}

pub(crate) struct Rpc {
    client: Client,
    url: Url,
}

impl Rpc {
    pub(crate) fn new(endpoint: &str) -> Result<Self> {
        let url = loopback_url(endpoint)?;
        let client = Client::builder()
            .no_proxy()
            .redirect(Policy::none())
            .user_agent("solana-kit-verification")
            .connect_timeout(Duration::from_secs(1))
            .timeout(Duration::from_secs(5))
            .build()?;
        Ok(Self { client, url })
    }

    fn reply(&self, method: &str, parameters: Value) -> Result<Value> {
        let body = json!({"jsonrpc":"2.0","id":1,"method":method,"params":parameters});
        let response = self.client.post(self.url.clone()).json(&body).send()?;
        require(response.status().is_success(), "validator HTTP status")?;
        let mut bytes = Vec::new();
        response.take(REPLY_BYTES + 1).read_to_end(&mut bytes)?;
        require(
            bytes.len() as u64 <= REPLY_BYTES,
            "validator response limit",
        )?;
        let reply: Value = serde_json::from_slice(&bytes)?;
        require(
            reply["id"] == 1 && reply["jsonrpc"] == "2.0",
            format!("{method} returned an invalid JSON-RPC envelope: {reply}"),
        )?;
        Ok(reply)
    }

    pub(crate) fn healthy(&self) -> Result<bool> {
        let reply = self.reply("getHealth", json!([]))?;

        if reply.get("error").is_none() && reply["result"] == "ok" {
            return Ok(true);
        }

        if reply.get("result").is_none() && reply["error"]["code"] == -32005 {
            return Ok(false);
        }

        Err(Error::Response(format!(
            "invalid validator health response: {reply}"
        )))
    }

    pub(crate) fn call(&self, method: &str, parameters: Value) -> Result<Value> {
        let reply = self.reply(method, parameters)?;
        require(
            reply.get("error").is_none(),
            format!("{method} failed: {reply}"),
        )?;
        reply
            .get("result")
            .cloned()
            .ok_or_else(|| Error::Response(format!("{method} omitted its result")))
    }

    pub(crate) fn balance(&self, account: &str) -> Result<u64> {
        self.call("getBalance", json!([account,{"commitment":"confirmed"}]))?["value"]
            .as_u64()
            .ok_or_else(|| Error::Response("balance is not an unsigned integer".into()))
    }
}
