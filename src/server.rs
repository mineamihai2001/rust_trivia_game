mod lib;

use serde_json::Value;

use crate::lib::Headers;
use std::{
    collections::HashMap,
    error::Error,
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    str, thread,
};

const MESSAGE_SIZE: usize = 64;
const QUESTIONS_SIZE: usize = 64;

const DEFAULT_POINTS: u16 = 10;

struct ApiResult {
    category: String,
    _type: String,
    difficulty: String,
    question: String,
    correct_answer: String,
    incorrect_answers: Vec<String>,
}

struct ApiResponse {
    response_code: i32,
    results: Vec<ApiResult>,
}

struct Level {
    index: i32,
    question: String,
    answer: String,
    points: u16,
}

impl Level {
    fn new() -> Self {
        let data = Level::fetch_data().unwrap();
        Self {
            index: 0,
            question: data["results"][0]["question"]
                .to_string()
                .replace("&quot;", "\""),
            answer: data["results"][0]["correct_answer"]
                .to_string()
                .replace("&quot;", "\""),
            points: DEFAULT_POINTS,
        }
    }

    fn fetch_data() -> Result<Value, Box<dyn Error>> {
        let response = reqwest::blocking::get("https://opentdb.com/api.php?amount=1&category=9")?
            .json::<serde_json::Value>()
            .unwrap();
        Ok(response)
    }

    fn get_question(&self) -> &String {
        &self.question
    }

    fn validate_answer(&self, answer: &String) -> bool {
        self.answer.eq(answer)
    }

    fn get_answer(&self) -> &String {
        &self.answer
    }
}

fn serialize_headers<'a>(headers: &Headers) -> [u8; 256] {
    let serialized = match serde_json::to_string(headers) {
        Ok(value) => value,
        Err(_) => "".to_string(),
    };
    let mut buffer = [0 as u8; 256];
    let array_tmp = serialized.as_bytes();
    buffer[0..array_tmp.len()].clone_from_slice(array_tmp);
    buffer
}

fn handle_client(mut stream: TcpStream) {
    let level = Level::new();

    let question = level.get_question();
    println!(" >> Question {}", question);

    let bytes_question = question.as_bytes();

    let headers = Headers {
        buffer_size: bytes_question.len(),
    };
    let buffer_headers = serialize_headers(&headers);

    stream.write(&buffer_headers).unwrap();
    stream.write(bytes_question).unwrap();
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                println!("Error (connection failed): {}", e);
            }
        }
    }
    drop(listener);
}
