# Credit to Roger Torres for the Dockerfile and explanation
# https://dev.to/rogertorres/first-steps-with-docker-rust-30oi

FROM rust:1.73 as build

# create a new empty shell project
RUN USER=root cargo new --bin dozy-host
WORKDIR /dozy-host

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/dozy_host*
RUN cargo build --release

# our final base
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /dozy-host/target/release/dozy-host .

# set the startup command to run your binary
CMD ["./dozy-host"]
