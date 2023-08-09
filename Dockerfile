FROM rust:latest AS builder
WORKDIR /website_v1
COPY ./ .
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder /website_v1/target/release/app /
ENTRYPOINT ["/app"]
