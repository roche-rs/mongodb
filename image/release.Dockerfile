FROM quay.io/roche/rust:1.49.0-alpine

COPY . /app-build

WORKDIR "/app-build"

ENV RUSTFLAGS="-C target-feature=-crt-static" 

RUN \
  apk add --no-cache musl-dev openssl-dev && \
  cargo build --release && cargo test default \
 && echo "#!/bin/sh" > run.sh \
 && bin=$(find ./target/release -maxdepth 1 -perm -111 -type f| head -n 1) \
 && echo ./${bin##*/} >> run.sh \
 && chmod 755 run.sh

RUN rm -f target/release/deps/roche*