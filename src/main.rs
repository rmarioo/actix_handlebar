
mod parse_json;

#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate serde_json;

use actix_web::{web, App, HttpResponse, HttpServer};

use handlebars::Handlebars;

use std::io;

// Macro documentation can be found in the actix_web_codegen crate
#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "name": "Handlebars"
    });
    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[get("/{user}/{data}")]
async fn user(
    hb: web::Data<Handlebars<'_>>,
    web::Path(info): web::Path<(String, String)>,
) -> HttpResponse {


    let full_name = parse_json::find_person("data/persons.json", &info.0)
        .expect("error loading file data/persons.json")
        .map_or(String::from("not found"), |v| v.last_name);

    let data = json!({
        "user": info.0,
        "data": full_name
    });
    let body = hb.render("user", &data).unwrap();

    HttpResponse::Ok().body(body)
}



#[actix_web::main]
async fn main() -> io::Result<()> {
    // Handlebars uses a repository for the compiled templates. This object must be
    // shared between the application threads, and is therefore passed to the
    // Application Builder as an atomic reference-counted pointer.
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(index)
            .service(user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
