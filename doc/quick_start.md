# Quick Start

## 开发服务器

1. 安装 `Docker` 和 `Docker Composer`(后者大多会在安装 `Docker` 本身时被自动安装)

1. 运行 `docker-composer up`

## 运行单体服务(热重载)
以 `xxx-serve` 举例

1. 安装 [rust toolchain](https://www.rust-lang.org/tools/install) 和 [cargo watch](https://lib.rs/crates/cargo-watch)

1. `cd ./xxx-serve`

1. `cargo watch -x run`

## 运行前端开发服务器(热重载)

1. 安装 [rust toolchain](https://www.rust-lang.org/tools/install) 和 [trunk](https://trunkrs.dev/)

1. `cd ./page-serve/web/`

1. `trunk serve`