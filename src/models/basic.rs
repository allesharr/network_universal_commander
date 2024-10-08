use axum::*;
use dotenv::*;
use serde::{Deserialize, Serialize, Serializer};
use sqlx::{PgPool, FromRow};
use std::{fs::File, io::{self, BufWriter, Read, Write}, net::SocketAddr};
use tokio::net::TcpStream;
use dotenv::dotenv;
use std::env;
use bcrypt::{hash, DEFAULT_COST};
use std::fs;

#[derive(Serialize, Deserialize, Default,Debug, FromRow)]
pub struct DeviceMap {
    pub devices:Vec<Device>
}
    
// Структура для устройства
#[derive(Serialize, Deserialize, FromRow, Default, Debug)]
pub struct Device {
    pub ip: String,
    pub open_ports: Vec<i32>,
    //размещение на карте
    pub x_param: i32,
    pub y_param:i32,
    pub admin_username: String,
    pub admin_password_hash: String,
}

impl Device {
    fn x_param(&self) -> i32 {
        self.x_param
    }
    
    fn y_param(&self) -> i32 {
        self.y_param
    }
}
// Структура для получаемых данных
#[derive(Deserialize, Default, Debug)]
pub struct DevicePayload {
    pub  ip: String,
    pub  admin_username: String,
    pub  admin_password: String,
}

fn start() {

}
