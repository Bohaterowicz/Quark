#[macro_use]
extern crate actix_web;

mod quark_db;
mod quark_post_data;

use std::{io, sync::Mutex};
use actix_web::{http::{header}, web::{self, Data}, App, HttpServer, HttpResponse, HttpRequest};
use actix_cors::Cors;
use serde::{Serialize, Deserialize};
use quark_post_data::{QuarkPost, QuarkPostPrototype};
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::{Pool, PooledConnection};

#[derive(Serialize)]
struct PostQuarkPostResponse {
    response: String
}

#[derive(Serialize)]
struct QuarkPostsResponse {
    posts: Vec<QuarkPost>,
}
#[derive(Serialize)]
struct PeekQuarkPostsResponse {
    count: i64,
}

#[derive(Deserialize)]
struct GetQuarkPostsQuery {
    most_recent_id: Option<i64>,
    current_post_count: i32,
    new_post_request_count: i32,
}

#[derive(Deserialize)]
struct GetNewQuarkPostsId {
    id: i64,
}

#[derive(Deserialize)]
struct  PeekNewQuarkPostsId {
    id: i64,
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().insert_header(header::ContentEncoding::Identity).body("OK")
}

#[get("/posts/new/peek")]
async fn peek_new_quark_posts(req: HttpRequest) -> HttpResponse {
    let server_state = req.app_data::<Data<QuarkServerState>>().unwrap();
    let db_connection = server_state.get_db_connection();
    let query_params = 
        web::Query::<PeekNewQuarkPostsId>::from_query(req.query_string()).unwrap();

    let most_recent_id = query_params.id;
    let new_post_count = quark_db::peek_new_post_count(db_connection, most_recent_id);

    let response = PeekQuarkPostsResponse{
        count: new_post_count,
    };

    HttpResponse::Ok().json(response)
}

#[get("/posts/new")]
async fn get_new_quark_posts(req: HttpRequest) -> HttpResponse {
    let server_state = req.app_data::<Data<QuarkServerState>>().unwrap();
    let db_connection = server_state.get_db_connection();

    let query_params =
        web::Query::<GetNewQuarkPostsId>::from_query(req.query_string()).unwrap();   
    let most_recent_id = query_params.id;

    let new_quark_posts = quark_db::get_new_posts_from_id(db_connection, most_recent_id);

    let response = QuarkPostsResponse{
        posts: new_quark_posts,
    };

    HttpResponse::Ok().json(response)
}

#[get("/posts")]
async fn get_quark_posts(req: HttpRequest) -> HttpResponse {
    let server_state = req.app_data::<Data<QuarkServerState>>().unwrap();
    let db_connection = server_state.get_db_connection();

    let http_query_parameters = 
        web::Query::<GetQuarkPostsQuery>::from_query(req.query_string()).unwrap();

    let quark_posts = quark_db::get_posts_from_offset(db_connection, 
        http_query_parameters.new_post_request_count, 
        http_query_parameters.current_post_count, 
        http_query_parameters.most_recent_id);

    let response = QuarkPostsResponse{
        posts: quark_posts.unwrap(),
    };

    HttpResponse::Ok().json(response)
}

#[post("/posts")]
async fn add_quark_post(req: HttpRequest, post: web::Json<QuarkPostPrototype>) -> HttpResponse {
    let server_state = req.app_data::<Data<QuarkServerState>>().unwrap();
    let db_connection = server_state.get_db_connection();

    let post: QuarkPostPrototype = post.into_inner();

    match quark_db::insert_into_posts_table(&db_connection, &vec![post]) {
        Ok(new_post) => {
            return HttpResponse::Ok().json(new_post);
        }
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Error when inserting new post: {}", e));
        }
    }
}

struct QuarkServerState {
    db_connection_pool: Mutex<Pool<SqliteConnectionManager>>,
}

impl QuarkServerState {
    fn get_db_connection(&self) -> PooledConnection<SqliteConnectionManager> {
        self.db_connection_pool.lock().unwrap().get().unwrap()
    }
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    println!("Hello Quark!");

    let manager = SqliteConnectionManager::file("quark.db");
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let server_state = Data::new(QuarkServerState {
        db_connection_pool: Mutex::new(pool),
    });
    
    if !quark_db::init_db(server_state.get_db_connection()) {
        println!("Database initialization failed, shutting down...");
        return Ok(());
    } 

    HttpServer::new(move || {
        App::new()
        .app_data(server_state.clone())
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
        .service(get_new_quark_posts)
        .service(peek_new_quark_posts)
    })
    .bind(("127.0.0.1", 1234))?
    .run()
    .await
}
