FROM rust:latest AS builder
WORKDIR /website_v1
COPY ../.. .
RUN cargo build --bin hengus_website --release

FROM gcr.io/distroless/cc:nonroot
COPY --from=builder --chown=nonroot /website_v1/target/release/hengus_website /usr/local/bin/
COPY --from=builder --chown=nonroot /website_v1/assets/hengus /website_v1/assets/hengus
WORKDIR /website_v1
USER nonroot
ENTRYPOINT ["hengus_website"]
