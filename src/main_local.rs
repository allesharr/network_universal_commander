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



static mut Devices: DeviceMap = DeviceMap {
    devices: Vec::new()
};

#[derive(Serialize, Deserialize, Default,Debug, FromRow)]
struct DeviceMap {
    devices:Vec<Device>
}
    
// Структура для устройства
#[derive(Serialize, Deserialize, FromRow, Default, Debug)]
struct Device {
    ip: String,
    open_ports: Vec<i32>,
    //размещение на карте
    x_param: i32,
    y_param:i32,
    admin_username: String,
    admin_password_hash: String,
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
struct DevicePayload {
    ip: String,
    admin_username: String,
    admin_password: String,
}

pub fn start() {
    start_server();
}
#[tokio::main]
pub async fn start_server() {
    dotenv::dotenv().ok();
    let work_status = env::var("WORK_STATUS").unwrap_or(String::from("local"));
    let str_status = work_status.as_str();
    println!("{}",work_status );

    match str_status {
        "local" => {
            begin_local_server()
        }
        "network" => {
            begin_network_server()
        }
        _ => {

        }
    }
}

fn begin_local_server() {
    let filename = "map_data.json";
    load_map(filename);
}

fn load_map(f_name: &str) -> DeviceMap {
    let path = "./map_data.json";
        let file_res = File::open(path);
        let file = match file_res {
            Ok(value) => value,
            Err(error) => panic!("Error happaned: {}!", error)
        };

        let data_res = fs::read_to_string(path);
        let data = match data_res {
            Ok(value) => value,
            Err(err) => panic!("{}",err)
        };
        
        let result_struct = serde_json::from_str(&data.as_str()).unwrap();
        return result_struct;
}
/// .
///
/// # Локальное сохранение файла
///
/// 
fn save_map_local_json(data: DeviceMap) {
    let _buf: Vec<u8> = Vec::new();
    let path = "./map_data.json";
    let mut write_file = File::create(path).expect("cannot create file");
    let res = serde_json::to_string(&data).unwrap();
    let byte_res = res.as_bytes();
    let _ = write_file.write(byte_res);

    // let data = Devices.serialize(write_file);
    
}

fn begin_network_server() {

}

#[cfg(test)]
mod tests{
    use std::fs;

    use super::*;
    #[test]
    fn test_save_data() {
        let dev = Device{
            ip: String::from("127.0.0.1"),
            ..Default::default()
        };
        let mut devices = DeviceMap{
            ..Default::default()
        };

        devices.devices.push(dev);
        save_map_local_json(devices);
    }

    #[test]
    fn load_data() {
        let path = "./map_data.json";
        let map = load_map(path);
        println!("{:?}", map)
    }
}

