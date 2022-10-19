# login-serve

```yaml
labels:
    - "traefik.http.routers.to-login.rule=PathPrefix(`/login/request`)"
```

## login up

- **接口地址**
    ```c
    /login/request/login_up/:username<String>/:password(blake2b256)<Vec<hex>>/:timestamp<f64>
    ```

- **请求方式:** post

- **返回格式:** success "SCSS"

- **Error:**
  - "USERNAME_ALREADY_EXISTS": 在数据库里发现同样的用户名
  - "SERVER_SIDE_ERROR": 发生服务端错误

- 备注
    - 校验时间戳
    - 入库 username, password, 注册日期

## login in

```c
/login/request/login_in
    /:username<String>
    /:password(blake2b_psw_mac mix by timestamp)<Vec<hex>>
    /:timestamp<f64>
```

- **请求方式:** post

- **返回格式:** JWT

- **Error:**
  - "INCORRECT_USERNAME_OR_PASSWORD": 错误的用户名或密码
  - "REQUEST_TIME_OUT": 请求超时
  - "CLI_SIDE_ERROR": 发生客户端错误
  - "SERVER_SIDE_ERROR": 发生服务端错误

- 备注
    - 校验时间戳
    - 生成活动凭证
    - 返回 JWT, 持久化凭证