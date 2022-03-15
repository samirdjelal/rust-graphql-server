mod health;

use async_graphql::*;
use health::{
	health_mutation::HealthMutation,
	health_query::HealthQuery
};

#[derive(MergedObject, Default)]
pub struct Query(HealthQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(HealthMutation);

pub type SchemaType = Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema() -> Schema<Query, Mutation, EmptySubscription> {
	Schema::build(Query::default(), Mutation::default(), EmptySubscription::default()).finish()
}