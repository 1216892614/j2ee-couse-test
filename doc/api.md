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