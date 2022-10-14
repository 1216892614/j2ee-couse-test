# This Object

这是一个 full rust stack 微服务架构的 Web 服务 Demo

## 项目目录结构

```sh
.
├── README.md
├── Cargo.toml          # 定义了项目的 [workspace]
├── docker-compose.yml  # 用于运行完整的开发调试服务器
├── book.toml           # 定义了项目文档构建
├── doc                 # 项目文档(不包括 CaaD)
│   │
│  ...
│
├── login-serve         # 登录服务
│   │
│  ...
│
├── page-serve          # 页面服务
│   │
│  ...
│   └── Web             # 站点前端开发
│
└──...
```

## 网关

网关使用 [traefik](https://doc.traefik.io/traefik/), 自动监听服务进行反向代理