mod schema;

extern crate pretty_env_logger;

use actix_web::web::Data;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use crate::schema::SchemaType;

async fn index(schema: web::Data<SchemaType>, req: GraphQLRequest) -> GraphQLResponse {
	schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
	let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
	Ok(HttpResponse::Ok()
		.content_type("text/html; charset=utf-8")
		.body(source))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	std::env::set_var("RUST_LOG", "debug"); // debug, trace
	pretty_env_logger::init();
	
	let schema_object = schema::build_schema();
	
	println!("Playground: http://localhost:8000");
	
	HttpServer::new(move || {
		App::new()
			.app_data(Data::new(schema_object.clone()))
			.service(web::resource("/").guard(guard::Post()).to(index))
			.service(web::resource("/").guard(guard::Get()).to(index_playground))
	})
		.bind("127.0.0.1:8000")?
		.workers(10)
		.run()
		.await
}