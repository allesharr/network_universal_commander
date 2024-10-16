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
use crate::models::basic::*;
use crate::models::database::*;

#[derive(Serialize)]
struct MyResponse {
    message: String,
}

async fn my_handler() -> impl IntoResponse {
    let responce = MyResponse {
        message: "Hello, World!".to_string(),
    };
    (StatusCode::OK, Json(responce))
}

#[tokio::main]
async fn start_sever(map : DeviceMap) {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/handler", get(move || async { my_handler().await })
    );
    let server_socket = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let _server = axum::serve(server_socket, app);
    _server.await.unwrap();
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn simple_server(){
        // start_sever();
    }
}



