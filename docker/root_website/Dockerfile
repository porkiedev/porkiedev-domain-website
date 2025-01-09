FROM rust:latest AS builder
WORKDIR /website_v1
COPY ../.. .
RUN cargo build --bin root_website --release

FROM gcr.io/distroless/cc:nonroot
COPY --from=builder --chown=nonroot /website_v1/target/release/root_website /usr/local/bin/
USER nonroot
ENTRYPOINT ["root_website"]
