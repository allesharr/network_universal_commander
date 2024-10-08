use axum::{
    body::Bytes,
    error_handling::HandleErrorLayer,
    extract::{DefaultBodyLimit, Path, State},
    handler::Handler,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Router
};
use serde::{Deserialize, Serialize};
use std::{net::{SocketAddr, TcpListener}, sync::Arc};
use std::str::FromStr;
use std::time::Duration;
use axum::Json;
use tokio::time::sleep;

#[derive(Deserialize, Serialize)]
struct InputData {
    test: String,
    id: i32
}
// Handler for the GET request at "/"
#[derive(Serialize)]
struct Message {
    message: String
}
enum ApiResponse {
    OK,
    Created,
    JsonData(Vec<Message>),
}
async fn hello_world() -> &'static str {
    "Hello world!"
}struct AppState {
    // ...
}

#[derive(Serialize,Deserialize)]
struct CreateUser {
    email: String,
    password: String,
}
static  mut users :Vec<CreateUser> = Vec::new(); 
async fn show_users(Json(payload): Json<CreateUser>) {

    let data = format!("email: {}, password: {}", payload.email, payload.password);
    println!("{}", data);
}
#[derive(Deserialize)]
struct Pagination {
    page: usize,
    per_page: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self { page: 1, per_page: 30 }
    }
}
#[tokio::main]
async fn start_sever() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let server_socket = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let _server = axum::serve(server_socket, app);
    _server.await.unwrap();
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn simple_server(){
        start_sever();
    }
}



