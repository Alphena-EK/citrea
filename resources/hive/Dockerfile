FROM rust:1.81 AS runtime
WORKDIR /app


# Install system dependencies
RUN apt-get update && apt-get -y upgrade && \
    apt-get install -y libclang-dev pkg-config && \
    apt-get install protobuf-compiler -y && apt-get install -y curl && \
    apt-get install cmake -y


COPY . .

# Build the project
RUN SKIP_GUEST_BUILD=1 cargo build --release --bin citrea


EXPOSE 8545


ENTRYPOINT ["sh", "-c", "./target/release/citrea --genesis-paths ./resources/genesis/mock-dockerized --rollup-config-path ./resources/configs/mock-dockerized/sequencer_rollup_config.toml --sequencer ./resources/configs/mock-dockerized/sequencer_config.toml"]

