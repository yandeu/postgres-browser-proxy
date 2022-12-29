use std::{
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

#[derive(Debug)]
pub enum RequestError {
    MaxPayloadSize,
}

pub fn parse_request(stream: &mut TcpStream) -> Result<(String, Vec<u8>), RequestError> {
    const MAX_PAYLOAD_SIZE: u32 = 1024 * 1024 * 12; // 413 Payload Too Large

    let mut http_request = String::new();
    let mut buf_reader = BufReader::new(stream.try_clone().unwrap());
    loop {
        let line = buf_reader.read_line(&mut http_request).unwrap();
        if line < 3 {
            //detect empty line
            break;
        }
    }
    let http_request: Vec<_> = http_request.lines().collect();

    let request_line = http_request.first().unwrap().to_string();
    let content_length: usize = {
        let mut size: usize = 0;
        for line in http_request {
            if line.starts_with("Content-Length") {
                size = line
                    .split(':')
                    .last()
                    .unwrap()
                    .trim()
                    .parse::<usize>()
                    .unwrap();
            }
        }
        size
    };

    if content_length > MAX_PAYLOAD_SIZE.try_into().unwrap() {
        return Err(RequestError::MaxPayloadSize);
    }

    let mut body = vec![0; content_length];
    buf_reader.read_exact(&mut body).unwrap();

    Ok((request_line, body))
}
