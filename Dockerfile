FROM rust:1.51-buster as builder
ENV USER root
RUN apt-get update && apt-get install -y build-essential curl gcc sshguard musl musl-dev ca-certificates git libssl-dev musl-tools zlib1g-dev openssl pkg-config

# Install OpenSSL
WORKDIR /tmp
RUN git clone --depth 1 --branch OpenSSL_1_1_1g https://github.com/openssl/openssl.git && \
    cd openssl && \
    ./config zlib '-Wl,-rpath,$(LIBRPATH)' && \
    make && make test && make install && ldconfig -v

WORKDIR /
ENV OPENSSL_DIR="/tmp/openssl"

RUN ln -s /usr/local/lib /tmp/openssl/lib

RUN rustup target add x86_64-unknown-linux-musl
WORKDIR app
COPY Cargo.toml .
RUN mkdir src \
    && echo "// dummy file" > src/lib.rs \
    && cargo build
RUN rm -rf src
COPY src src
RUN cargo build --target x86_64-unknown-linux-musl  --release

FROM scratch
COPY credentials.yaml /credentials.yaml
COPY --from=builder app/target/x86_64-unknown-linux-musl/release/template /template
CMD ["/template"]
