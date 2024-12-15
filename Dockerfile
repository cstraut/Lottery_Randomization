########################################################################################################################
# lottery randomization build stage
########################################################################################################################

FROM rust:1.83.0-slim as build

RUN rustup target add lottery_randomization && \
    apt update && \
    apt install -y musl-tools musl-dev && \
    update-ca-certificates

COPY ./src ./src
COPY ./Cargo.lock .
COPY ./Cargo.toml .

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid 10001 \
    "rustdev"

RUN cargo build --target lottery_randomization --release

########################################################################################################################
# lotter randomization image
########################################################################################################################

FROM scratch

COPY --from=build /etc/passwd /etc/passwd
COPY --from=build /etc/group /etc/group

COPY --from=build --chown=rustdev:rustdev ./target/x86_64-unknown-linux-musl/release/lotter_randomization /app/lottery_randomization

USER rustdev:rustdev

ENTRYPOINT ["./app/rustdev"]
