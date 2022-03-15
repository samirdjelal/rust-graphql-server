use async_graphql::*;

#[derive(Default)]
pub struct HealthMutation;

#[Object]
impl HealthMutation {
	/// Returns `true` to signify that the GraphQL server is reachable.
	async fn set_health(&self) -> bool {
		false
	}
}