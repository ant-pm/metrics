FROM --platform=$BUILDPLATFORM rust:latest AS builder

ARG TARGETPLATFORM

RUN case "$TARGETPLATFORM" in \
    "linux/amd64") echo "x86_64-unknown-linux-gnu"  > /target ;; \
    "linux/arm64") echo "aarch64-unknown-linux-gnu" > /target ;; \
    esac

RUN apt-get update && apt-get install -y \
    gcc-x86-64-linux-gnu \
    gcc-aarch64-linux-gnu \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add $(cat /target)

ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc


WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN TARGET=$(cat /target) && \
    mkdir src && echo 'fn main(){}' > src/main.rs && \
    cargo build --release --target $TARGET && \
    rm -rf src
COPY src ./src
RUN TARGET=$(cat /target) && \
    touch src/main.rs && \
    cargo build --release --target $TARGET

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/*/release/backend /usr/local/bin/backend
EXPOSE 3000
CMD ["/usr/local/bin/backend"]
