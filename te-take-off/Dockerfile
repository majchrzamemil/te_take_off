FROM rust:1.67
WORKDIR /app
RUN cargo init
COPY Cargo.toml /app/Cargo.toml
RUN cargo fetch
RUN apt update && apt install lld clang pkg-config -y
COPY . /app
RUN cargo clean
RUN cargo install sqlx-cli
# RUN cargo sqlx prepare --database-url=postgres://postgres:password@172.17.0.2:5432/postgres?sslmode=disable
ENV DATABASE_URL=postgres://postgres:password@172.17.0.2:5432/postgres?sslmode=disable
# ENV SQLX_OFFLINE true
# CMD ["sqlx", "migrate", "run", "databae-url", "postgres://poestgres:password@172.17.0.2:5432/postgres"]
RUN env DATABASE_URL=postgres://postgres:password@172.17.0.2:5432/postgres?sslmode=disable cargo build --release 
EXPOSE 8000
# RUN cargo run --release -
ENTRYPOINT ["./target/release/te-take-off"]