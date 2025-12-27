FROM rust:slim
COPY ./app /app
WORKDIR /app
RUN apt update && apt install -y libsqlite3-dev libssl-dev pkg-config
