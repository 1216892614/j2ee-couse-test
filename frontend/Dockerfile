FROM rust:1.63 as builder

COPY /conf /tmp/
RUN cd ~ \
    && mv /tmp/conf/config /usr/local/cargo/ \
    && export RUSTUP_DIST_SERVER="https://rsproxy.cn" \
    && export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup" \
    && cargo install wasm-bindgen-cli \
    && cargo install trunk \
    && cargo new --bin frontend 
WORKDIR /root/frontend
ADD ./* ./*
RUN cargo build --release \
    && trunk build --release

FROM nginx

WORKDIR /usr/share/nginx

COPY --from=builder ./dist /usr/share/nginx/html
COPY --from=builder /tmp/conf/nginx.conf /etc/nginx/nginx.conf

ENTRYPOINT [ "./nginx" ]