
FROM clux/muslrust:stable as builder
RUN groupadd -g 10001 -r dockergrp && useradd -r -g dockergrp -u 10001 crabby

COPY Cargo.lock .
COPY Cargo.toml .
# just to make cargo install dependencies first without compiling whole source code.
RUN mkdir src \
    && echo "fn main(){println!(\"mpa at bots.house\");}" > src/main.rs
RUN set -x && cargo build --target x86_64-unknown-linux-musl --release
RUN set -x && rm target/x86_64-unknown-linux-musl/release/deps/emojeez*

# add the real code and compile the project
COPY src ./src
RUN set -x && cargo build --target x86_64-unknown-linux-musl --release
RUN mkdir -p /rbin
RUN set -x && cp target/x86_64-unknown-linux-musl/release/emojeez /rbin/

# second stage
FROM scratch

COPY --from=0 /etc/passwd /etc/passwd
USER crabby

COPY --from=0 /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs

ENV RUST_LOG="error,emojeez=info"
COPY --from=builder /rbin/emojeez /

CMD ["/emojeez"]
