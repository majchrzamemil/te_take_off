FROM rust:1.67 
WORKDIR /app
RUN cargo init
COPY Cargo.toml /app/Cargo.toml
RUN cargo fetch
RUN apt update && apt install lld clang pkg-config -y
COPY . /app
RUN cargo clean
RUN cargo install sqlx-cli
EXPOSE 8000
# RUN cargo sqlx prepare --database-url=postgres://postgres:password@172.17.0.2:5432/postgres?sslmode=disable
# ENV DATABASE_URL=postgres://postgres:password@172.17.0.2:5432/postgres?sslmode=disable
# ENV SQLX_OFFLINE true
# CMD ["sqlx", "migrate", "run", "databae-url", "postgres://poestgres:password@172.17.0.2:5432/postgres"]
RUN env DATABASE_URL=postgres://postgres:password@localhost:8015/ cargo build --release

CMD ["/app/target/release/te-take-off"]




# ARG APP_NAME
# ARG BUILD_VERSION="local"
# ARG BASE_IMAGE=debian:11.4-slim
# ARG BUILDER_IMAGE=lukemathwalker/cargo-chef:latest-rust-1.67-slim
# ARG CARGO_REGISTRY_AUTH_URL

# FROM ${BUILDER_IMAGE} as custom_builder

# # RUN git config --global credential.helper store && echo "${CARGO_REGISTRY_AUTH_URL}" > ~/.git-credentials

# ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

# FROM custom_builder as builder

# ARG APP_NAME
# ARG BUILD_REVISION
# # ARG SS_CACHE_REDIS=$SSCACHE_REDIS

# WORKDIR /build
# COPY . .
# # COPY .cargo/config .cargo/config

# # RUN echo "[build]" >> .cargo/config 

# RUN cargo build --release -p te-take-off

# FROM ${BASE_IMAGE} as runtime
# ARG APP_NAME

# COPY --from=builder /build/target/release/${APP_NAME} /app/te-take-off
# # COPY --from=builder /build/services/${APP_NAME}/config.yaml /app/configs/config.yaml
# COPY --from=builder /lib/x86_64-linux-gnu/libz* /lib/x86_64-linux-gnu/

# WORKDIR /app
# ENTRYPOINT [ "/app/te-take-off" ]