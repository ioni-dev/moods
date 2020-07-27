# Build environment
FROM ekidd/rust-musl-builder:latest AS builder

# Add our source code
ADD . ./

# setting permissions on source code
RUN sudo chown -R rust:rust /home/rust

# Build our application
RUN cargo build --release

# Building docker container
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/moods \
    /usr/local/bin
CMD /usr/local/bin/moods
