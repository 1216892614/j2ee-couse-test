FROM rust:1.63.0-buster as builder

COPY /conf/* /tmp/

RUN cd ~ \
  && mv /tmp/config /usr/local/cargo/ \
  && export RUSTUP_DIST_SERVER="https://rsproxy.cn" \
  && export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup" \
  && rustup target add wasm32-unknown-unknown \
  && cargo install wasm-bindgen-cli \
  && cargo install trunk \
  && cargo new --bin frontend

WORKDIR /root/frontend

COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release \
  && rm src/*.rs target/release/deps/frontend*

ADD . ./

RUN cargo build --release \
  && trunk build --release

FROM nginx

COPY --from=builder /root/frontend/dist /usr/share/nginx/html
COPY --from=builder /tmp/nginx.conf /etc/nginx/nginx.conf