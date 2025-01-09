FROM rust:latest AS builder
WORKDIR /website
COPY ../.. .
RUN cargo build --bin website_root --release

FROM gcr.io/distroless/cc:nonroot
COPY --from=builder --chown=nonroot /website/target/release/website_root /usr/local/bin/
COPY --from=builder --chown=nonroot /website/assets/root /website/assets/root
WORKDIR /website
USER nonroot
ENTRYPOINT ["website_root"]
