pub mod handlers;

use async_graphql::{Schema, EmptyMutation, EmptySubscription, Object, Context, Result};

pub use handlers::health_handler;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn ping(&self, _ctx: &Context<'_>) -> Result<String> {
        Ok("pong".to_string())
    }
}

pub type GatewaySchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema() -> GatewaySchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_graphql::{Request};

    #[tokio::test]
    async fn graphql_ping_works() {
        let schema = build_schema();
        let resp = schema.execute(Request::new("{ ping }")).await;
        let data = resp.data.into_json().expect("graphql response data JSON");
        assert_eq!(data["ping"], "pong");
    }
}
