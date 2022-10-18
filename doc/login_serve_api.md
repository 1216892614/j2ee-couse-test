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

- **返回格式:** success "SCSS", error "{msg}"

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

- 备注
    - 校验时间戳
    - 生成活动凭证
    - 返回 JWT, 持久化凭证