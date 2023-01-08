#[macro_use]
extern crate actix_web;


use std::io;
use actix_web::{http::header, web::{self}, App, HttpServer, HttpResponse};
use actix_cors::Cors;
use serde::{Serialize, Deserialize};

static mut DB: Option<sqlite::Connection> = None;

#[derive(Serialize, Deserialize)]
struct QuarkPost {
    name: String,
    text: String,
}


#[derive(Serialize)]
struct PostQuarkPostResponse {
    response: String
}

#[derive(Serialize)]
struct QuarkPostsResponse {
    posts: Vec<QuarkPost>,
}



#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().insert_header(header::ContentEncoding::Identity).body("OK")
}

#[get("/posts")]
async fn get_quark_posts() -> HttpResponse {

    let mut quark_posts = Vec::new();
    unsafe {
        match &DB {
            Some(db) =>{
                let query = "SELECT * FROM posts";
                db
                .iterate(query, |pairs| {
                    let post = QuarkPost {
                        name: String::from(pairs[0].1.unwrap()),
                        text: String::from(pairs[1].1.unwrap()),
                    };
                    quark_posts.push(post);
                    true
                })
                .unwrap();
            },
            None => {panic!("db failed...")},
        }
    }

    let response = QuarkPostsResponse{
        posts: quark_posts,
    };

    HttpResponse::Ok().json(response)
}

#[post("/posts")]
async fn add_quark_post(post: web::Json<QuarkPost>) -> HttpResponse {
    unsafe {
        match &DB {
            Some(db) =>{
                let dpost: QuarkPost = post.into_inner();
                let query = "INSERT INTO posts VALUES(?, ?)";
                let mut statement = db.prepare(query).unwrap();
                statement.bind((1, dpost.name.as_str())).unwrap();
                statement.bind((2, dpost.text.as_str())).unwrap();
                statement.next().unwrap();
            },
            None => {panic!("db failed...")},
        }
    }

    HttpResponse::Ok().finish()
}

fn init_db() {
    let connection = sqlite::open("qark.db").unwrap();
    
    let query = "
    CREATE TABLE IF NOT EXISTS posts (name TEXT, post_text TEXT);
    INSERT INTO posts VALUES ('Bohaterowicz', 'asdasdasdasdasadasdasdas');
    INSERT INTO posts VALUES ('testUser69', 'LOLOLOLOLOLOLOLOLOLOLOLOLOLLOLOL!');
    ";
    
    connection.execute(query).unwrap();
    
    unsafe {DB = Some(connection);}
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    println!("Hello!");
    
    init_db();

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
        .service(add_quark_post)
    })
    .bind(("127.0.0.1", 1234))?
    .run()
    .await
}