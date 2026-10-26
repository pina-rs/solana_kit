//! Exact endpoint mappings retain opaque route identities outside request bodies.

use super::{
    CreateWebhookRequest, DeleteWebhookRequest, GetAllWebhooks, GetWebhookRequest, ToggleWebhook,
    UpdateWebhookRequest, Webhook, response,
};
use crate::service::{Body, Error, Operation, Service, Verb};

fn path(identity: &str) -> Vec<String> {
    vec!["v0".into(), "webhooks".into(), identity.into()]
}

impl Operation for CreateWebhookRequest {
    type Response = Webhook;
    const SERVICE: Service = Service::Webhooks;
    const VERB: Verb = Verb::Post;
    const BODY: Body = Body::Json;

    fn path(&self) -> Vec<String> {
        vec!["v0".into(), "webhooks".into()]
    }

    fn decode(&self, bytes: &[u8]) -> Result<Webhook, Error> {
        response::decode(bytes)
    }
}

impl Operation for GetAllWebhooks {
    type Response = Vec<Webhook>;
    const SERVICE: Service = Service::Webhooks;
    const VERB: Verb = Verb::Get;
    const BODY: Body = Body::Absent;

    fn path(&self) -> Vec<String> {
        vec!["v0".into(), "webhooks".into()]
    }

    fn decode(&self, bytes: &[u8]) -> Result<Vec<Webhook>, Error> {
        response::decode(bytes)
    }
}

impl Operation for GetWebhookRequest {
    type Response = Webhook;
    const SERVICE: Service = Service::Webhooks;
    const VERB: Verb = Verb::Get;
    const BODY: Body = Body::Absent;

    fn path(&self) -> Vec<String> {
        path(&self.webhook_id)
    }

    fn decode(&self, bytes: &[u8]) -> Result<Webhook, Error> {
        response::decode(bytes)
    }
}

impl Operation for UpdateWebhookRequest {
    type Response = Webhook;
    const SERVICE: Service = Service::Webhooks;
    const VERB: Verb = Verb::Put;
    const BODY: Body = Body::Json;

    fn path(&self) -> Vec<String> {
        path(&self.webhook_id)
    }

    fn decode(&self, bytes: &[u8]) -> Result<Webhook, Error> {
        response::decode(bytes)
    }
}

impl Operation for ToggleWebhook {
    type Response = Webhook;
    const SERVICE: Service = Service::Webhooks;
    const VERB: Verb = Verb::Patch;
    const BODY: Body = Body::Json;

    fn path(&self) -> Vec<String> {
        path(&self.webhook_id)
    }

    fn decode(&self, bytes: &[u8]) -> Result<Webhook, Error> {
        response::decode(bytes)
    }
}

impl Operation for DeleteWebhookRequest {
    type Response = bool;
    const SERVICE: Service = Service::Webhooks;
    const VERB: Verb = Verb::Delete;
    const BODY: Body = Body::Absent;

    fn path(&self) -> Vec<String> {
        path(&self.webhook_id)
    }

    fn decode(&self, bytes: &[u8]) -> Result<bool, Error> {
        response::payload(bytes)?;
        Ok(true)
    }
}
