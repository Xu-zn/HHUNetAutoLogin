use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;

/// 连接信息
#[derive(Deserialize, Serialize)]
pub struct Connection {
    /// 连接地址
    pub url: String,
    /// 连接值
    pub value: String,
    /// 等待时间
    pub wait_seconds: u64,
}


pub fn check_network(connection: &Connection, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let res = client.get(&connection.url).send()?;
    if res.status().is_success() && res.text()?.eq(&connection.value) {
        Ok(())
    } else {
        Err("网络未连接".into())
    }
}