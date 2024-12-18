# 河海大学校园网自动登录

## 使用方法

1. 下载并解压
2. 运行`auto-login.exe`
3. 在托盘图标上点击右键，选择`退出`以退出程序

## 配置文件

配置文件为`config.toml`，需要与`auto-login.exe`在同一目录下

格式为TOML格式，具体格式如下：

```toml
[login]
username = # 学号
password = # 密码
service = "_service_1" # 服务，0为校园网，1为南京移动，2为常州电信，3为常州联通
wait_seconds = 3 # 浏览器打开网页后的等待时间，防止页面元素加载不完全

[connection]
url = # 连接检查地址，即程序会通过GET请求该地址，如果返回值与value相同，则认为网络已连接
value = # 连接检查值
wait_seconds = 60 # 连接检查间隔时间
```

> 本质上是模拟人操作浏览器进行登陆
