use std::fs;

use crate::http::{ParseError, Request, Response, StatusCode, self};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, err: &ParseError) -> Response {
        println!("Failed to parse request: {}", err);
        Response::new(crate::http::StatusCode::BadRequest, None)
    }
}

pub struct WebsiteHandler{
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        WebsiteHandler{
            public_path
        }
    }

    pub fn read_file(&self, file_path:&str) -> Option<String> {
        let path = format!("{}/{}",self.public_path,file_path);
        println!("reading file: {}", path);
        match fs::canonicalize(path){
            Ok(path) => {
                if path.starts_with(&self.public_path){
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory traversal attack attempted!:{}", file_path);
                    None
                }
            },
            Err(_) => None,
        }
        
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &http::Request) -> http::Response {
        match request.method(){
            "GET" => {
                match request.path() {
                    "/"=> Response::new(StatusCode::Ok, self.read_file("index.html")),
                    "/hello" => http::Response::new(
                        crate::http::StatusCode::Ok,
                        Some("<h1>Hello World!<h1>".to_string())),
                    path => match self.read_file(path){
                        Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                        None => Response::new(StatusCode::NotFound, None),
                    }
                }
            }
            _ => Response::new(StatusCode::NotFound, None),
        }
    }

    fn handle_bad_request(&mut self, err: &http::ParseError) -> http::Response {
        println!("Failed to parse request: {}", err);
        http::Response::new(crate::http::StatusCode::BadRequest, None)
    }
}
