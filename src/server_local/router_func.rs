use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::models::basic::DeviceMap;





async fn GetDevices() -> impl IntoResponse {
    let devices = 
    devices = DeviceMap {
        devices: Vec::new()
    };
    (StatusCode::OK, Json(devices))
}