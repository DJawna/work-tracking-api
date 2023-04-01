

use actix_web::{ App, HttpServer,  Error, web::{self, Json}};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::DateTime;
use chrono::offset::Utc;
use std::str::FromStr;
use utoipa::{ToSchema, OpenApi};
use utoipa_swagger_ui::SwaggerUi;
//use std::From

#[derive(OpenApi)]
#[openapi(paths(get_project), components(schemas(Project)))]
struct ApiDoc;

#[derive(Serialize, Deserialize, ToSchema)]
struct Project{
    id: Uuid,
    name: String,
    customer_id: Uuid,
    project_description: String,
    created: DateTime<Utc>,
    last_modified: DateTime<Utc>
}

/* 
#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}*/

async fn get_doc() -> String{
    ApiDoc::openapi().to_pretty_json().unwrap()
}

#[utoipa::path(
    get,
    path = "/project",
    responses(
        (status = 200, description = "Projects according to filter criteria", body = Vec<Project>)
    ),
    params(
        ("search_by_name"= String, Query, description = "search string containing the start of the projectname.")
    )
)]
async fn get_project(search_by_name: String) -> Result<Json<Vec<Project>>, Error> {
    let result = vec!(
        Project{
        
        id : Uuid::new_v4(),
        
        customer_id: Uuid::new_v4(),
        
        created: DateTime::from_str("2022-01-01T01:01:01Z").unwrap(),
        last_modified: DateTime::from_str("2023-01-01T01:01:01Z").unwrap(),
        
        name : String:: from("the new project"),
        project_description: String::from("description of the project")
    });
    Ok( Json( result))
} 

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        //.service(greet)
        .service(
            web::resource("/api-docs/openapi.json")
            .route(web::get().to(get_doc))
        )
        .service(
            web::resource("/project")
            .route(web::get().to(get_project))
        )
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}")
            
            .url("/api-docs/openapi.json", ApiDoc::openapi())
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
