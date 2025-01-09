FROM rust:latest AS builder
WORKDIR /website
COPY ../.. .
RUN cargo build --bin website_hengus --release

FROM gcr.io/distroless/cc:nonroot
COPY --from=builder --chown=nonroot /website/target/release/website_hengus /usr/local/bin/
COPY --from=builder --chown=nonroot /website/assets/hengus /website/assets/hengus
WORKDIR /website
USER nonroot
ENTRYPOINT ["website_hengus"]
