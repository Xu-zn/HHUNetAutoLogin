use serde::{Deserialize, Serialize};
use std::{thread::sleep, time::Duration};
use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::env::current_dir;

/// 配置登录信息
#[derive(Deserialize, Serialize)]
pub struct LoginInfo {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
    /// 服务
    pub service: String,
    /// 等待时间
    pub wait_seconds: u64,
}


pub fn login(login_info: &LoginInfo) -> Result<(), Box<dyn std::error::Error>> {
    let chrome_path = current_dir()?.join("chrome-win").join("chrome.exe");
    let launch_options = LaunchOptionsBuilder::default()
        .path(Some(chrome_path))
        .headless(true)
        .sandbox(false)
        .build()?;
    let browser = Browser::new(launch_options)?;
    let tab = browser.new_tab()?;

    let open_url = tab.navigate_to("http://eportal.hhu.edu.cn");

    if let Err(e) = open_url {
        return Err(e.into());
    }
    sleep(Duration::from_secs(login_info.wait_seconds));
    let find_logout = tab.find_element("#toLogOut");

    if find_logout.is_ok() {
        return Ok(());
    }

    let input_username = tab.find_element("#username");
    match input_username {
        Ok(el) => {
            // 需要先点一下输入框外面的地方，否则无法输入
            tab.find_element("#username_tip")?.click()?;
            el.type_into(&login_info.username)?;
        },
        Err(e) => {
            return Err(e.into());
        }
    }

    let input_password = tab.find_element("#pwd");
    match input_password {
        Ok(el) => {
            // 需要先点一下输入框外面的地方，否则无法输入
            tab.find_element("#pwd_tip")?.click()?;
            el.type_into(&login_info.password)?;
        },
        Err(e) => {
            return Err(e.into());
        }
    }

    let input_service = tab.find_element("#selectDisname");

    match input_service {
        Ok(el) => {
            // 先点一下，否则无法选择
            el.click()?;
            // 选择服务
            let selector = format!("#{}", &login_info.service);
            tab.find_element(&selector)?.click()?;
        },
        Err(e) => {
            return Err(e.into());
        }
    }

    tab.find_element("#loginLink_div")?.click()?;

    tab.wait_for_element_with_custom_timeout("#toLogOut", Duration::from_secs(login_info.wait_seconds + 5))?;

    Ok(())
}