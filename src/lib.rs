use domains::identities;
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
            let identities_context = identities::Context::new(ctx);

            let result = identities_context.create().await?;
            let response = serde_json::to_string(&result)?;
            Response::ok(response)
        })
        .run(req, env)
        .await
}
