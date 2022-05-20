####################################################################################################
## Rust Web Builder
####################################################################################################
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=cgn-social
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /cgn-social

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## FrontEnd React Builder
####################################################################################################

FROM node:14-alpine AS builder2
WORKDIR /cgn-social/web
COPY ./web/package.json ./
RUN yarn install
COPY ./web/yarn.lock ./
COPY ./web .
RUN yarn build

####################################################################################################
## Final image
####################################################################################################
FROM alpine:latest

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /cgn-social

# Copy our build
COPY --from=builder /cgn-social/target/x86_64-unknown-linux-musl/release/cgn-social ./
COPY --from=builder2 /cgn-social/web/build ./web/build

# Use an unprivileged user.
USER cgn-social:cgn-social

CMD ["/cgn-social/cgn-social"]