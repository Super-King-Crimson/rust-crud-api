mod request;

use request::*;

use postgres::{Client, NoTls, Error as PostgresError};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};

// const DB_URL: &str = env!("DATABASE_URL");
const DB_URL: &str = "DATABASE_URL";

const HTTP_OK: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const HTTP_NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const HTTP_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

impl User {
    fn new(id: i32, name: String, email: String) -> User {
        User {
            id: Some(id),
            name,
            email
        }
    }
}

pub fn set_database() -> Result<(), PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;
    
    client.batch_execute("CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        )"
    )?;

    Ok(())
}

pub fn start(port: &str) {
    let listener = TcpListener::bind(port).unwrap();
    println!("Started listening on {port}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_request(stream),
            Err(e) => println!("Error: {e}"),
        }
    }
}

fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut req = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            req.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match req.as_str() {
                req if req.starts_with("POST /users") => request::handle_post_request(req),
                req if req.starts_with("GET /users/") => handle_get_request(req),
                req if req.starts_with("GET /users") => handle_get_all_request(req),
                req if req.starts_with("PUT /users/") => handle_put_request(req),
                req if req.starts_with("DELETE /users/") => handle_delete_request(req),
                _ => (HTTP_NOT_FOUND.to_string(), "Not Found".to_string()),
            };

            let res = format!("{}{}", status_line, content);
            let res = res.as_bytes();
            stream.write_all(res).unwrap();
        }
        Err(e) => println!("Error: {e}")
    }
}

fn get_id(req: &str) -> &str {
    req.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

fn parse_user_from_req(req: &str) -> Result<User, serde_json::Error> {
    serde_json::from_str(req.split("\r\n\r\n").last().unwrap_or_default())
}