FROM rust:latest AS builder
WORKDIR /website_v1
COPY ./ .
RUN cargo build --release

FROM gcr.io/distroless/cc:nonroot
COPY --from=builder --chown=nonroot /website_v1/target/release/app /usr/local/bin/
USER nonroot
ENTRYPOINT ["app"]
