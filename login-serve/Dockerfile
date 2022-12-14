FROM rust:1.63.0-buster as builder

COPY /conf/* /tmp/
RUN cd ~ \
    && mv /tmp/config /usr/local/cargo/ \
    && export RUSTUP_DIST_SERVER="https://rsproxy.cn" \
    && export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup" \
    && cargo new login-serve
WORKDIR /root/login-serve

COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release \
  && rm src/*.rs target/release/deps/login_serve*

ADD . ./
RUN cargo build --release

FROM debian:buster-slim

ARG APP=/usr/src/app
COPY --from=builder /tmp/sources.list /etc/apt/sources.list
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

#test
EXPOSE 3030

ENV TZ=Etc/UTC \
    APP_USER=appuser
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /root/login-serve/target/release/login-serve ${APP}/login-serve

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./login-serve"]


