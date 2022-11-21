FROM rust:1.64-slim-buster 
RUN cargo new --bin tp02
WORKDIR /tp02
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
COPY ./src ./src
RUN rm ./target/release/deps/Tp01*
RUN cargo build --release
RUN useradd -r -ms  /bin/bash --system random
USER random
CMD "./target/release/Tp01"