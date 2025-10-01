FROM rust:latest

RUN rustup update stable
RUN rustup default stable

WORKDIR /usr/src/app

COPY Cargo.toml ./

COPY ./src ./src

RUN cargo install --path .

RUN cargo build

COPY . .

EXPOSE 8080

CMD [ "cargo", "run" ]
