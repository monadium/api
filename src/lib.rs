use serde::Serialize;
use worker::*;

mod utils;

#[derive(Serialize)]
struct CreatedResponse {
    jwt: String,
}

#[event(fetch, respond_with_errors)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    utils::set_panic_hook();

    let router = Router::new();

    router
        .post_async("/identities/create", |_req, ctx| async move {
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
