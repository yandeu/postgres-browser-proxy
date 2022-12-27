#![allow(clippy::unused_io_amount)]
use clap::Parser;
use r2d2_postgres::{
    postgres::{Client, NoTls},
    PostgresConnectionManager,
};
use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    thread,
};
use threadpool::ThreadPool;

mod args;
mod http;
mod image;
mod parse;
mod query;
mod types;

static INDEX_HTML: &str = include_str!("files/index.html");
static QUERY_JS: &str = include_str!("files/query.js");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn handle_connection(mut stream: TcpStream, client: &mut types::PgClient, args: crate::args::Args) {
    let (request_line, body) = crate::http::parse_request(&mut stream);

    let (status_line, body, content_type): (&str, String, &str) =
        if request_line == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", INDEX_HTML.to_string(), "text/html")
        } else if request_line.starts_with("GET /query.js") {
            (
                "HTTP/1.1 200 OK",
                QUERY_JS.to_string().replace("3000", args.port()),
                "application/javascript",
            )
        } else if request_line.starts_with("POST /query") {
            let query = body;
            match query::make_query(query, client) {
                Ok(data) => {
                    let json = parse::row_to_string(data);
                    ("HTTP/1.1 200 OK", json, "application/json")
                }
                Err(e) => ("HTTP/1.1 400 Bad Request", e, "text/plain"),
            }
        } else {
            (
                "HTTP/1.1 404 NOT FOUND",
                "404 NOT FOUND".to_string(),
                "text/plain",
            )
        };

    let contents = body;
    let length = contents.len();

    let response = format!("{status_line}\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: {content_type}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    println!("** POSTGRES BROWSER PROXY **");
    println!("version: {}", VERSION);
    println!("source: https://github.com/yandeu/postgres-browser-proxy");
    println!();

    // image test
    crate::image::crop_image();

    let args = crate::args::Args::parse();

    // check db connection
    let db_string = args.to_db_string();
    thread::spawn(move || {
        match Client::connect(&db_string, NoTls) {
            Ok(client) => client.close().unwrap(),
            Err(_e) => println!("WARNING: No connection to db!"),
        };
    });

    let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port())).unwrap();
    println!("Running on http://localhost:{}/", args.port());

    let manager = PostgresConnectionManager::new(args.to_db_string().parse().unwrap(), NoTls);

    let pg_pool = r2d2::Pool::new(manager).unwrap();
    let thread_pool = ThreadPool::new(4);

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
