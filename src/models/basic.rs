use axum::*;
use dotenv::*;
use serde::{Deserialize, Serialize, Serializer};
use sqlx::{PgPool, FromRow};
use std::{borrow::BorrowMut, fs::File, io::{self, BufWriter, Read, Write}, net::SocketAddr};
use tokio::net::TcpStream;
use dotenv::dotenv;
use std::env;
use bcrypt::{hash, DEFAULT_COST};
use std::fs;

#[derive(Serialize, Deserialize, Default,Debug, FromRow)]
pub struct DeviceMap {
    pub devices:Vec<Device>
}



impl DeviceMap {
    pub fn GetDeviceByIp(self, ip: String) -> Option<Device>{
        for dev in self.devices {
            if dev.ip == ip {
                return Some(dev)
            }
        }
        return None;
    }

    pub fn GetDevicesByOpenPort(self, ports: Vec<i32>) -> Vec<Device>{
        let mut res = Vec::new();
        for dev in self.devices{
            for port in &dev.open_ports{
                if ports.contains(&port) {
                    res.push(dev.clone());
                }
            }
        }
        return res;
    }

    pub fn ChangeAuthDataForDevice(old_hash:String, new_hash:String) -> bool {
        
        todo!()
    }

    
}
    
// Структура для устройства
#[derive(Serialize, Deserialize, FromRow, Default, Debug, Clone)]
pub struct Device {
    ip: String,
    open_ports: Vec<i32>,
    //размещение на карте
    x_param: i32,
    y_param:i32,
    admin_username: String,
    admin_password_hash: String,
}


enum DeviceConnections {
    GetDeviceOS(),
    GetDeviceIP(),
    GetDeviceStatus(), 

}

impl Device {
    pub fn ChangeHashPassword( mut self, new_hash_password: String){
        // Need to use try. 
        let  old_hash = self.admin_password_hash;

        let is_password_changed = &self.try_change_password(new_hash_password);
        
        match is_password_changed {
            Some(value) => {
                if value == &true {
                    let mut current = self.borrow_mut();
                    current.admin_password_hash = new_hash_password;
                }
            },
            None => {
                todo!()
            }
        }
        
       
        
        todo!()
    }
    pub fn ChangeDeviceIp(new_ip: String) {
        todo!()
    }
    fn try_change_password(self, new_hash_password: String) -> Option<bool>{
        todo!()
    }

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
