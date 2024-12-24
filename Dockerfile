########################################################################################################################
# lottery randomization build stage
########################################################################################################################

FROM rust:latest AS build

RUN rustup target add x86_64-unknown-linux-musl && \
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

RUN cargo build --target x86_64-unknown-linux-musl --release

########################################################################################################################
# lotter randomization image
########################################################################################################################

FROM rust:latest

COPY --from=build /etc/passwd /etc/passwd
COPY --from=build /etc/group /etc/group

COPY --from=build --chown=rustdev:rustdev ./target/x86_64-unknown-linux-musl/release/lottery_randomization /app/lottery_randomization

USER rustdev:rustdev

ENTRYPOINT ["./app/lottery_randomization"]
