#[macro_use]
extern crate actix_web;

use std::io;
use actix_web::{http::header, web::{self}, App, HttpServer, HttpResponse, HttpRequest};
use actix_cors::Cors;
use serde::{Serialize, Deserialize};
use sqlite::Statement;

#[derive(Deserialize)]
struct QuarkPostPrototype {
    username: String,
    post_content: String,
    post_attachments: String,
}

#[derive(Serialize, Deserialize)]
struct QuarkPost {
    id: i64,
    username: String,
    post_content: String,
    post_attachments: String,
    post_time: String,
}

#[derive(Serialize)]
struct PostQuarkPostResponse {
    response: String
}

#[derive(Serialize)]
struct QuarkPostsResponse {
    posts: Vec<QuarkPost>,
}

#[derive(Deserialize)]
struct GetQuarkPostsQuery {
    most_recent_id: Option<i64>,
    current_post_count: i32,
    new_post_request_count: i32,
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().insert_header(header::ContentEncoding::Identity).body("OK")
}

#[get("/posts")]
async fn get_quark_posts(req: HttpRequest) -> HttpResponse {
    let query_parameters = 
        web::Query::<GetQuarkPostsQuery>::from_query(req.query_string()).unwrap();

    let connection = sqlite::open("quark.db").unwrap();
    let mut statement: Statement;
    if query_parameters.most_recent_id.is_some() {
        let query = "SELECT * FROM posts WHERE id <= :start_id ORDER BY id DESC LIMIT :limit OFFSET :offset";
        let start_id = query_parameters.most_recent_id.unwrap();
        statement = connection.prepare(query).unwrap();
        statement.bind((":start_id", start_id)).unwrap();
    }
    else {
        let query = "SELECT * FROM posts ORDER BY id DESC LIMIT :limit OFFSET :offset";
        statement = connection.prepare(query).unwrap();
    }
    
    let post_offset: i64 = query_parameters.current_post_count as i64;
    let post_limit: i64 = query_parameters.new_post_request_count as i64;
    statement.bind_iter::<_, (_, sqlite::Value)>([
        (":limit", post_limit.into()),
        (":offset", post_offset.into()),
    ]).unwrap();

    let mut quark_posts = Vec::new();

    while let Ok(sqlite::State::Row) = statement.next() {
        let post = QuarkPost {
            id: statement.read::<i64, _>("id").unwrap(),
            username: statement.read::<String, _>("username").unwrap(),
            post_content: statement.read::<String, _>("post_content").unwrap(),
            post_attachments: statement.read::<String, _>("post_attachments").unwrap(),
            post_time: statement.read::<String, _>("post_time").unwrap(),
        };
        quark_posts.push(post);
    }

    let response = QuarkPostsResponse{
        posts: quark_posts,
    };

    HttpResponse::Ok().json(response)
}

#[post("/posts")]
async fn add_quark_post(post: web::Json<QuarkPostPrototype>) -> HttpResponse {
    let connection = sqlite::open("quark.db").unwrap();
    let post: QuarkPostPrototype = post.into_inner();

    match insert_into_posts_table(&connection, &vec![post]) {
        Ok(_) => {}
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Error when inserting new post: {}", e));
        }
    }

    HttpResponse::Ok().finish()
}

fn insert_into_posts_table(connection: & sqlite::Connection, posts: &Vec<QuarkPostPrototype>) -> Result<(), sqlite::Error> {
    
    fn create_insert_values(posts: &Vec<QuarkPostPrototype>) -> String {
        let mut result = String::new();
        
        for post in posts{
            let value = format!("('{}', datetime('now'), '{}', '{}'),", post.username, post.post_content, post.post_attachments);
            result.push_str(value.as_str());
        }
    
        result.pop();
    
        result
    }
    
    let insert_values = create_insert_values(&posts);
    let query = format!("INSERT INTO posts(username, post_time, post_content, post_attachments) VALUES {}", insert_values);
    connection.execute(query)?;
    Ok(())
}

fn create_posts_table(connection: & sqlite::Connection) -> Result<(), sqlite::Error> {
    let query = "
        CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            post_time DATETIME NOT NULL,
            post_content TEXT NOT NULL,
            post_attachments TEXT NOT NULL
        );
    ";
    connection.execute(query)?;
    Ok(())
}

fn insert_test_posts(connection: & sqlite::Connection) -> Result<(), sqlite::Error> {
    let mut test_posts = vec![];
    let post = QuarkPostPrototype{
        username: String::from("Bohaterowicz"),
        post_content: String::from("This is a test message. This is a test message."),
        post_attachments: String::from("")
    };
    test_posts.push(post);
    let post = QuarkPostPrototype{
        username: String::from("testUser69"),
        post_content: String::from("This is another test message! This is another test message!"),
        post_attachments: String::from("")
    };
    test_posts.push(post);

    insert_into_posts_table(connection, &test_posts)?;

    Ok(())
}

fn init_db() -> bool {
    let connection = sqlite::open("quark.db").unwrap();
    match create_posts_table(&connection) {
        Ok(_) => 
            match insert_test_posts(&connection) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {}", e);
                    return false;
                }
            }
        Err(e) => {
            println!("Error: {}", e);
            return false;
        }
    }

    // TODO: add custom error type for error handlig (QarkError) 
    return true;
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    println!("Hello!");
    
    if !init_db() {
        println!("Database initialization failed, shutting down...");
        return Ok(());
    } 

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
