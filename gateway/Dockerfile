FROM rust:1.63.0-buster as builder

COPY /conf/* /tmp/
RUN cd ~ \
    && mv /tmp/config /usr/local/cargo/ \
    && export RUSTUP_DIST_SERVER="https://rsproxy.cn" \
    && export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup" \
    && cargo new gateway
WORKDIR /root/gateway

COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release \
  && rm src/*.rs target/release/deps/gateway*

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

COPY --from=builder /root/gateway/target/release/gateway ${APP}/gateway
COPY --from=builder /root/gateway/gateway.toml ${APP}/gateway.toml

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./gateway"]


