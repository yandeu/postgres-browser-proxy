#![allow(clippy::unused_io_amount)]
use clap::Parser;
use http::RequestError;
use r2d2_postgres::{
    postgres::{Client, NoTls},
    PostgresConnectionManager,
};
use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    thread,
};

mod args;
mod http;
mod image;
mod long_lat;
mod parse;
mod query;
mod types;

static INDEX_HTML: &str = include_str!("files/index.html");
static QUERY_JS: &str = include_str!("files/query.js");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn crate_response(status_line: &str, body: String, content_type: &str) -> String {
    let length = body.len();
    format!("{status_line}\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: {content_type}\r\nContent-Length: {length}\r\n\r\n{body}")
}

fn handle_connection(mut stream: TcpStream, client: &mut types::PgClient, args: crate::args::Args) {
    let parse_result = crate::http::parse_request(&mut stream);

    if parse_result.is_err() {
        let (status_line, body) = match parse_result.err().unwrap() {
            RequestError::MaxPayloadSize => ("HTTP/1.1 413 Payload Too Large", "PAYLOAD TOO LARGE"),
        };
        let response = crate_response(status_line, body.to_string(), "text/plain");
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    }

    let (request_line, body) = parse_result.unwrap();

    let (status_line, body, content_type): (&str, String, &str) =
        if request_line.starts_with("GET / HTTP/") {
            ("HTTP/1.1 200 OK", INDEX_HTML.to_string(), "text/html")
        } else if request_line.starts_with("GET /query.js") {
            (
                "HTTP/1.1 200 OK",
                QUERY_JS.to_string().replace("3000", args.port()),
                "application/javascript",
            )
        } else if request_line.starts_with("POST /query") {
            let query = std::str::from_utf8(&body).unwrap();
            match query::make_query(query, client) {
                Ok(data) => {
                    let json = parse::row_to_string(data);
                    ("HTTP/1.1 200 OK", json, "application/json")
                }
                Err(e) => ("HTTP/1.1 400 Bad Request", e, "text/plain"),
            }
        } else if request_line.starts_with("POST /crop-image") {
            if body.is_empty() {
                (
                    "HTTP/1.1 400 Bad Request",
                    "Body is required".to_string(),
                    "text/plain",
                )
            } else {
                let image = std::str::from_utf8(&body).unwrap();
                match crate::image::crop_image(image) {
                    Ok(file) => ("HTTP/1.1 200 OK", file, "text/plain"),
                    Err(_e) => (
                        "HTTP/1.1 400 Bad Request",
                        String::from("Failed to crop image"),
                        "text/plain",
                    ),
                }
            }
        } else {
            (
                "HTTP/1.1 404 NOT FOUND",
                "404 NOT FOUND".to_string(),
                "text/plain",
            )
        };

    let response = crate_response(status_line, body, content_type);
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    println!("** POSTGRES BROWSER PROXY **");
    println!("version: {}", VERSION);
    println!("source: https://github.com/yandeu/postgres-browser-proxy");
    println!();

    let mut args = crate::args::Args::parse();

    // get environment variables (for docker)
    let env_vars = std::env::vars();
    for (key, value) in env_vars {
        match key.as_str() {
            "HOST" => args.set_host(value),
            "PG_HOST" => args.set_pg_host(value),
            _ => (),
        }
    }

    // check db connection
    let db_string = args.to_db_string();
    thread::spawn(move || {
        match Client::connect(&db_string, NoTls) {
            Ok(client) => client.close().unwrap(),
            Err(_e) => println!("WARNING: No connection to db!"),
        };
    });

    // adjust bind address
    let mut bind_address = "127.0.0.1";
    if args.host() == "0.0.0.0" {
        bind_address = args.host()
    }

    let listener = TcpListener::bind(format!("{}:{}", bind_address, args.port())).unwrap();
    println!("Running on http://{}:{}/", args.host(), args.port());
    println!(
        "Connecting to http://{}:{}/",
        args.pg_host(),
        args.pg_port()
    );

    let manager = PostgresConnectionManager::new(args.to_db_string().parse().unwrap(), NoTls);

    let pg_pool = r2d2::Pool::new(manager).unwrap();

    let thread_pool = threadpool::Builder::new()
        .thread_stack_size(1024 * 1024 * 16)
        .num_threads(4)
        .build();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let pg_pool = pg_pool.clone();
        let args = args.clone();
        thread_pool.execute(move || {
            let mut client: types::PgClient = pg_pool.get().unwrap();
            handle_connection(stream, &mut client, args);
        });
    }
}
