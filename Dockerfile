FROM rust:latest AS builder
WORKDIR /website_v1
COPY ./ .
RUN cargo build --release

FROM gcr.io/distroless/cc
WORKDIR /website_v1
COPY --from=builder /website_v1/target/release/website_v1 ./
ENTRYPOINT ["/website_v1/website_v1"]
