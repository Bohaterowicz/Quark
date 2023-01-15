extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

use std::vec;

use crate::quark_post_data::{QuarkPost, QuarkPostPrototype};
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::PooledConnection;
use rusqlite::{Rows, Statement};

pub fn peek_new_post_count(conn: PooledConnection<SqliteConnectionManager>, id: i64) -> i64 {
    let mut count: i64 = -1;
    
    let query = "SELECT COUNT(id) as count from posts WHERE id > :most_recent_id";
    let mut statement = conn.prepare(query).unwrap();
    
    let mut rows= statement.query(&[(":most_recent_id", id.to_string().as_str())]).unwrap();

    while let Some(row) = rows.next().unwrap() {
        count = row.get(0).unwrap();
    }

    count
}

pub fn get_new_posts_from_id(conn: PooledConnection<SqliteConnectionManager>, id: i64) -> Vec<QuarkPost> {
    let query = "SELECT * FROM posts WHERE id > :most_recent_id ORDER BY id DESC;";
    let mut statement = conn.prepare(query).unwrap();

    let id_idx = statement.column_index("id").unwrap();
    let username_idx = statement.column_index("username").unwrap();
    let post_time_idx = statement.column_index("post_time").unwrap();
    let post_content_idx = statement.column_index("post_content").unwrap();
    let post_attachments_idx = statement.column_index("post_attachments").unwrap();

    let posts_result = statement.query_map(&[(":most_recent_id", id.to_string().as_str())], |row| {
        Ok(QuarkPost {
            id: row.get(id_idx).unwrap(),
            username: row.get(username_idx).unwrap(),
            post_time: row.get(post_time_idx).unwrap(),
            post_content: row.get(post_content_idx).unwrap(),
            post_attachments: row.get(post_attachments_idx).unwrap(),
        })
    }).unwrap();

    let mut quark_posts = Vec::new();
    for post in posts_result {
        quark_posts.push(post.unwrap());
    }

    quark_posts
}

pub fn get_posts_from_offset(conn: PooledConnection<SqliteConnectionManager>, limit: i32, offset: i32, most_recent_id: Option<i64>) -> Result<Vec<QuarkPost>, rusqlite::Error> {
    let mut statement: Statement;
    let mut result: Rows;
    if most_recent_id.is_some() {
        let query = "SELECT * FROM posts WHERE id <= :start_id ORDER BY id DESC LIMIT :limit OFFSET :offset";
        let start_id = most_recent_id.unwrap();
        statement = conn.prepare(query).unwrap();
        result = statement.query(&[
            (":start_id", start_id.to_string().as_str()), 
            (":limit", limit.to_string().as_str()), 
            (":offset", offset.to_string().as_str())
            ])?;
    }
    else {
        let query = "SELECT * FROM posts ORDER BY id DESC LIMIT :limit OFFSET :offset";
        statement = conn.prepare(query).unwrap();
        result = statement.query(&[
            (":limit", limit.to_string().as_str()), 
            (":offset", offset.to_string().as_str())
            ])?;
    }

    let id_idx = result.as_ref().unwrap().column_index("id").unwrap();
    let username_idx = result.as_ref().unwrap().column_index("username").unwrap();
    let post_time_idx = result.as_ref().unwrap().column_index("post_time").unwrap();
    let post_content_idx = result.as_ref().unwrap().column_index("post_content").unwrap();
    let post_attachments_idx = result.as_ref().unwrap().column_index("post_attachments").unwrap();

    let mut quark_posts = Vec::new();
    while let Some(row) = result.next().unwrap() {
        let post = QuarkPost {
            id: row.get(id_idx).unwrap(),
            username: row.get(username_idx).unwrap(),
            post_time: row.get(post_time_idx).unwrap(),
            post_content: row.get(post_content_idx).unwrap(),
            post_attachments: row.get(post_attachments_idx).unwrap(),
        };
        quark_posts.push(post);
    }

    Ok(quark_posts)
}

pub fn insert_into_posts_table(connection: & PooledConnection<SqliteConnectionManager>, posts: &Vec<QuarkPostPrototype>) -> Result<QuarkPost, rusqlite::Error> {
    
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
    connection.execute(query.as_str(), [])?;

    let query = "SELECT * FROM posts ORDER BY id DESC LIMIT 1";
    let mut statement = connection.prepare(query)?;

    let id_idx = statement.column_index("id").unwrap();
    let username_idx = statement.column_index("username").unwrap();
    let post_time_idx = statement.column_index("post_time").unwrap();
    let post_content_idx = statement.column_index("post_content").unwrap();
    let post_attachments_idx = statement.column_index("post_attachments").unwrap();

    let mut rows = statement.query([])?;

    while let Some(row) = rows.next()? {
        let post = QuarkPost {
            id: row.get(id_idx).unwrap(),
            username: row.get(username_idx).unwrap(),
            post_time: row.get(post_time_idx).unwrap(),
            post_content: row.get(post_content_idx).unwrap(),
            post_attachments: row.get(post_attachments_idx).unwrap(),
        };
        return Ok(post)
    }

    Err(rusqlite::Error::QueryReturnedNoRows)

}

pub fn init_db(conn: PooledConnection<SqliteConnectionManager>) -> bool {

    fn create_posts_table(connection: & PooledConnection<SqliteConnectionManager>) -> Result<usize, rusqlite::Error> {
        let query = "
            CREATE TABLE IF NOT EXISTS posts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL,
                post_time DATETIME NOT NULL,
                post_content TEXT NOT NULL,
                post_attachments TEXT NOT NULL
            );
        ";
        connection.execute(query, [])
    }

    fn insert_test_posts(connection: & PooledConnection<SqliteConnectionManager>) -> Result<(), rusqlite::Error> {
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

    match create_posts_table(&conn) {
        Ok(_) => 
            match insert_test_posts(&conn) {
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