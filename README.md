### rcloudust

> 实现网盘功能

##### 使用 <a href="https://salvo.rs/" target="_blank">Salvo</a>(Rust后端) + <a href="https://framework7.io/" target="_blank">Framework7</a>(前端) + Sqlite3(使用rusqlite操作数据库,目前没发现比较好用的ORM框架)

|目录|说明|
|-|-|
|f7|前端项目|
|files|文件上传目录|
|src|后端代码|
|www|前端发布代码，后端项目中嵌入了该目录|

```
# 发布
cd f7 && npm run build          #前端页面发布
cd .. && cargo build --release
```