use domains::identities::{self, CreatedResponse};
use worker::*;

mod domain_schema;
mod domains;
mod utils;

#[event(fetch, respond_with_errors)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    utils::set_panic_hook();

    let router = Router::new();

    let identities_schema = identities::Context::schema();

    let urls = domain_schema::endpoint_urls(identities_schema);

    router
        .post_async(&urls[0], |_req, ctx| async move {
            // TODO: get the kv name by uppercasing identities_schema.name.
            // Since we are using move, it seems like it's not allowed right now.
            let kv = ctx.kv("IDENTITIES")?;

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

            let result = CreatedResponse {
                jwt: "#jwt#".into(),
            };

            let result = serde_json::to_string(&result)?;

            Response::ok(result)
        })
        .run(req, env)
        .await
}
