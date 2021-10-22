use serde::Serialize;

use crate::domain_schema::{Action, Intent, Schema};

#[derive(Serialize)]
pub struct CreatedResponse {
    pub jwt: String,
}

pub struct Context {}

impl Context {
    pub fn schema() -> Schema {
        Schema {
            name: "identities".to_string(),
            actions: vec![Action {
                action: "create".to_string(),
                intent: Intent::Command,
            }],
        }
    }
}
