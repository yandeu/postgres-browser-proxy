use std::{io::Read, net::TcpStream};

#[derive(Debug)]
pub enum RequestError {
    MaxPayloadSize,
}

pub fn parse_request(stream: &mut TcpStream) -> Result<(String, String), RequestError> {
    let mut data = String::new();
    {
        const MAX_PAYLOAD_SIZE: u32 = 1024 * 1024 * 5; // 413 Payload Too Large
        const CHUNK_SIZE: usize = 1024 * 2; // 2kb
        let mut bytes: u32 = 0;
        #[allow(unused_assignments)]
        let mut buffer = [0; CHUNK_SIZE];
        #[allow(unused_assignments)]
        let mut n: i32 = -1;

        loop {
            buffer = [0; CHUNK_SIZE];
            match stream.read(&mut buffer) {
                Ok(m) => {
                    data += std::str::from_utf8(&buffer).unwrap();
                    bytes += m as u32;
                    n = m as i32
                }
                Err(e) => {
                    println!("Stream error: {}", e);
                    n = 0
                }
            };
            if bytes >= MAX_PAYLOAD_SIZE {
                return Err(RequestError::MaxPayloadSize);
            }
            if n != CHUNK_SIZE as i32 {
                break;
            }
        }
    }
    let lines: Vec<String> = data.lines().map(|line| line.to_string()).collect();
    std::mem::drop(data);

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

    Ok((request_line, body))
}
