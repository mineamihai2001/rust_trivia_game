use std::{
    io::{Read, Write},
    net::TcpStream,
    str::from_utf8,
};

use rust_triva_game::Headers;

fn deserialize_headers(headers_buffer: [u8; 256]) -> Headers {
    let headers_string: &str = from_utf8(&headers_buffer).unwrap();
    let headers: Result<Headers, serde_json::Error> =
        serde_json::from_str(headers_string.replace("\0", "").as_str());
    headers.unwrap()
}

fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server to port 3333");

            'communication: loop {
                let mut headers_buffer = [0 as u8; 256];
                let mut headers: Headers = Headers { buffer_size: 0 };
                match stream.read_exact(&mut headers_buffer) {
                    Ok(_) => {
                        let response_headers = deserialize_headers(headers_buffer);
                        headers = response_headers;
                    }
                    Err(e) => {
                        println!("Failed to receive data {}", e);
                        stream
                            .shutdown(std::net::Shutdown::Both)
                            .expect("shutdown failed");
                    }
                }
                println!(" >> Headers from server :: {:?}", headers);

                let mut data: Vec<u8> = vec![0; headers.buffer_size];
                let mut message: &str = "";
                match stream.read_exact(&mut data) {
                    Ok(_) => {
                        let text = from_utf8(&data).unwrap();
                        message = text;
                    }
                    Err(e) => {
                        println!("Failed to receive data {}", e);
                        stream
                            .shutdown(std::net::Shutdown::Both)
                            .expect("shutdown failed");
                    }
                }
                println!(" >> Data received :: {}", message);
                break 'communication;
            }
        }
        Err(e) => {
            println!("Failed to connect to server: {}", e);
        }
    }

    println!("[INFO] - Connection to server terminated");
}
