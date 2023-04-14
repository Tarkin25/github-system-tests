FROM rust:1.68.1 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/app/target/release/github-system-tests /usr/local/bin/github-system-tests
COPY ./config /usr/local/bin/config
WORKDIR /usr/local/bin
CMD [ "./github-system-tests" ]