use crate::domain_schema::{Action, Intent, Schema};
use serde::Serialize;
use worker::{Result, RouteContext};

#[derive(Serialize)]
pub struct CreatedResponse {
    pub jwt: String,
}

pub struct Context {
    route_context: RouteContext<()>,
}

impl Context {
    pub fn new(route_context: RouteContext<()>) -> Self {
        Context { route_context }
    }

    pub fn schema() -> Schema {
        Schema {
            name: "identities".to_string(),
            actions: vec![Action {
                action: "create".to_string(),
                intent: Intent::Command,
            }],
        }
    }

    pub async fn create(&self) -> Result<CreatedResponse> {
        let kv = self.route_context.kv(&Self::schema().name.to_uppercase())?;

        kv.put(
            "fc82307b-2664-4a88-be26-38c1b50d0eac:1",
            r##"{
    "type": "CREATED",
    "version": 1,
    "inserted_at": "1634247836",
    "cid": "23d42487-d374-4df1-bebb-8a95428106d6",
    "id": "fc82307b-2664-4a88-be26-38c1b50d0eac",
    "data": {
        "email": "marcus@radell.net",
        "password_hash": "#password#",
    }
}"##,
        )?
        .execute()
        .await?;

        Ok(CreatedResponse {
            jwt: "#jwt#".into(),
        })
    }
}
