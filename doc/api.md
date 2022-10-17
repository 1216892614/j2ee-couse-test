# APIs

## 配置反向代理

- docker-compose
    ```yaml
    labels:
        - "traefik.http.routers.to-login.rule=PathPrefix('<你的路径前缀>')"
    ```

    - https 另设

    - 优先级只和规则长度有关

- Kubernetes
    <!-- TODO -->

## 规范

- 服务端只接受两位 Hex 编码的字符
    - 例如: "6a,6a,b6,bf,d6,6e,3e..."