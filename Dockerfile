FROM rust:1.88-bookworm AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        gnupg \
        lsb-release \
        wget \
    && wget -qO /etc/apt/trusted.gpg.d/php.gpg https://packages.sury.org/php/apt.gpg \
    && echo "deb https://packages.sury.org/php/ $(lsb_release -sc) main" > /etc/apt/sources.list.d/php.list \
    && apt-get update \
    && apt-get install -y --no-install-recommends \
        clang \
        libclang-dev \
        pkg-config \
        build-essential \
        php8.4-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM php:8.4-cli-bookworm

WORKDIR /app
COPY --from=builder /app/target/release/libgliner_rs_php.so /usr/local/lib/php/extensions/libgliner_rs_php.so
COPY models ./models
COPY docker/test.php ./test.php

RUN echo "extension=/usr/local/lib/php/extensions/libgliner_rs_php.so" > /usr/local/etc/php/conf.d/gliner_rs_php.ini

CMD ["php", "/app/test.php"]