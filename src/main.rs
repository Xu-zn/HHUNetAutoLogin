#![windows_subsystem = "windows"]

use reqwest::blocking::Client;
use std::time::Duration;
use std::env::current_dir;
use std::fs::File;
use serde::{Deserialize, Serialize};
use toml;
use tray_item::{IconSource, TrayItem};
use std::sync::mpsc::channel;
use std::thread;
use tklog::{info, error, LEVEL, LOG, Format, MODE};
use auto_login::{Connection, LoginInfo, check_network, login};


fn main() {

    // 设置日志
    LOG.set_console(false)  
       .set_level(LEVEL::Info)  
       .set_format(Format::LevelFlag | Format::Time)  
       .set_cutmode_by_time("auto-login.log", MODE::DAY, 7, false)
       .set_formatter("{level} {time}: {message}\n");

    // 加载配置

    let config_file = current_dir().unwrap().join("config.toml");
    let config_file = File::open(config_file);
    if config_file.is_err() {
        error!("配置文件不存在");
        return;
    }
    let config_string = std::io::read_to_string(config_file.unwrap());
    if config_string.is_err() {
        error!("配置文件读取失败");
        return;
    }
    let config: Config = toml::from_str(&config_string.unwrap()).unwrap();


    // 创建判断网络客户端
    let client = Client::new();

    // 创建托盘
    let mut tray = TrayItem::new("AutoLogin", IconSource::Resource("neco-icon")).unwrap();
    let (sender, receiver) = channel();
    let quit_sender = sender.clone();
    tray.add_menu_item("退出", move || {
        quit_sender.send(()).unwrap();
    }).unwrap();

    // 创建线程，循环检查网络，如果网络不通则登录
    let task = thread::spawn(move || loop {
        let connected = check_network(&config.connection, &client);
        if connected.is_ok() {
            info!("网络已连接");
        } else {
            let login_result = login(&config.login);
            if login_result.is_ok() {
                info!("登录成功");
            } else {
                error!("{}", login_result.err().unwrap());
            }
        }

        if receiver.recv_timeout(Duration::from_secs(config.connection.wait_seconds)).is_ok() {
            info!("准备退出");
            break;
        }
        
    });

    // 等待线程结束
    task.join().unwrap();

}


/// 配置信息
#[derive(Deserialize, Serialize)]
struct Config {
    /// 登录信息
    login: LoginInfo,
    /// 连接信息
    connection: Connection,
}






