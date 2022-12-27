use std::{io::Read, net::TcpStream};

pub fn parse_request(stream: &mut TcpStream) -> (String, String) {
    let mut buffer = [0; 65536];
    stream.read(&mut buffer).unwrap();
    let request_str = std::str::from_utf8(&buffer).unwrap();

    let lines: Vec<String> = request_str.lines().map(|line| line.to_string()).collect();
    let request_line = lines.first().unwrap().to_string();

    // get body
    let mut collect = false;
    let mut body = String::from("");
    for line in &lines {
        if collect {
            body.push_str(line);
        }
        if line.is_empty() {
            collect = true;
        }
    }
    body = body.trim_matches(char::from(0)).to_string();

    (request_line, body)
}
