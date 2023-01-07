#[macro_use]
extern crate actix_web;


use std::io;
use actix_web::{http::header, web::{self}, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;
use serde::Serialize;


#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().insert_header(header::ContentEncoding::Identity).body("OK")
}

#[derive(Serialize)]
struct QuarkPost {
    name: String,
    text: String,
}

#[derive(Serialize)]
struct QuarkPostsResponse {
    posts: Vec<QuarkPost>,
}

#[get("/posts")]
async fn get_quark_posts() -> Result<impl Responder, io::Error> {
    let quark_post1 = QuarkPost {
        name: String::from("Bohaterowicz"),
        text: String::from("ASDASDASDASDASDASDASDAS"),
    };

    let quark_post2 = QuarkPost {
        name: String::from("testUser123"),
        text: String::from("LALALALALALALALALALALALAL"),
    };

    let mut quark_posts = Vec::new();
    quark_posts.push(quark_post1);
    quark_posts.push(quark_post2);

    let response = QuarkPostsResponse{
        posts: quark_posts,
    };

    Ok(web::Json(response))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    println!("Hello!");
    HttpServer::new(|| {
        App::new()
        .wrap(
            Cors::default()
                .allowed_origin("http://localhost:8080")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                .allowed_header(header::CONTENT_TYPE)
                .supports_credentials()
                .max_age(3600),
        )
        .service(index)
        .service(get_quark_posts)
    })
    .bind(("127.0.0.1", 1234))?
    .run()
    .await
}