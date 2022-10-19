# page-serve

```yaml
labels:
    - "traefik.http.routers.to-page.rule=PathPrefix(`/`)"
```

## Get request

```
/:static path
```

映射 dist 里的静态资源

```
/**(任意其他路径)
```

返回 index.html

## No Found

```
/** -> /404
```