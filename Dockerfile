# Create the build container to compile the hello world program
FROM rust:1.51-buster as builder
ENV USER root
# RUN cargo new hello
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR app
COPY Cargo.toml .
COPY src src
RUN cargo build --target x86_64-unknown-linux-musl  --release


# RUN cargo build --release
# RUN ls

# CMD ["/hello"]


# # Create the execution container by copying the compiled hello world to it and running it
FROM scratch
COPY --from=builder app/target/x86_64-unknown-linux-musl/release/template /template
CMD ["/template"]
