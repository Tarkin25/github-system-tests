FROM rust:1.68.1 as builder
WORKDIR /usr/local/src
COPY . .
RUN cargo build --release
RUN rm -rf target/release/*.*
RUN find target/release -mindepth 1 -maxdepth 1 -type d -print0 | xargs -0 rm -rf 
RUN mv target/release/* ./app

FROM debian:buster-slim
COPY --from=builder /usr/local/src/app /usr/local/bin/app
COPY ./config /usr/local/bin/config
WORKDIR /usr/local/bin
CMD [ "./app" ]