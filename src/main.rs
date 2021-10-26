
mod parse_json;

#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate serde_json;

use actix_web::{middleware, web, App, HttpRequest,HttpResponse, HttpServer};
use std::env;
use handlebars::Handlebars;

use std::io;

// Macro documentation can be found in the actix_web_codegen crate
#[get("/")]
async fn index(req: HttpRequest,hb: web::Data<Handlebars<'_>>) -> HttpResponse {

    println!("REQ: {:?}", req);

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

    println!("i am startingggggg");
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    println!("host {}" , host);
    println!("port {}" , port);

    HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .app_data(handlebars_ref.clone())
            .service(index)
            .service(user)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await

}
