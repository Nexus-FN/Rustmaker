FROM rust:1.67-slim-buster as builder

RUN USER=root cargo new --bin matchmaker
WORKDIR ./matchmaker
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/matchmaker*
RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app