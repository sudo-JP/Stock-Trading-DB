FROM rust:1.67

WORKDIR /usr/src/app

COPY Cargo.toml ./

COPY ./src ./src

RUN cargo install --path .

RUN cargo build

COPY . .


CMD [ "cargo", "run" ]
