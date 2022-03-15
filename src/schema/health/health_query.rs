use async_graphql::*;

#[derive(Default)]
pub struct HealthQuery;

#[Object]
impl HealthQuery {
	/// Returns `true` to signify that the GraphQL server is reachable.
	async fn health(&self) -> bool {
		true
	}
}
