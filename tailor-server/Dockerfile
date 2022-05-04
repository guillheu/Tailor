FROM rust as build

MAINTAINER Guillaume Heu

WORKDIR /root/tailor-server
COPY src src
COPY Cargo.toml .
RUN cargo build -r --bins

FROM debian:stable-slim

RUN useradd -m tailor
WORKDIR /home/tailor/tailor-server/serve
COPY --from=build /root/tailor-server/target/release/tailor-server .
COPY examples ../examples
COPY examples/default .
RUN chown -R tailor ..
USER tailor

EXPOSE 8080

ENTRYPOINT ["/home/tailor/tailor-server/serve/tailor-server"]