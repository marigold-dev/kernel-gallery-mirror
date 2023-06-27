# Start with Ubuntu base image
FROM ubuntu:latest

# Set environment variables
ENV tezos_bin_output_dir=/root/tezos-bin

# Install necessary dependencies
RUN apt update && \
    apt install -y wget curl build-essential clang xxd wabt openssl pkg-config libssl-dev

# Download and setup Ligo 0.62
RUN wget https://gitlab.com/ligolang/ligo/-/jobs/3942174485/artifacts/raw/ligo

RUN mv ./ligo /usr/local/bin && chmod +x /usr/local/bin/ligo

# Download and setup Tezos bin
RUN mkdir -p ${tezos_bin_output_dir} && \
    curl --silent -L -o "${tezos_bin_output_dir}/binaries-17.0-rc1-1.tar.gz" https://github.com/serokell/tezos-packaging/releases/download/v17.0-rc1-1/binaries-17.0-rc1-1.tar.gz && \
    tar -xvf "${tezos_bin_output_dir}/binaries-17.0-rc1-1.tar.gz" -C ${tezos_bin_output_dir} && \
    chmod 777 -R ${tezos_bin_output_dir}

# Add Tezos bin to PATH
RUN echo 'export PATH="${tezos_bin_output_dir}:$PATH"' >> $HOME/.bashrc

# Install rustup
RUN curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh -s -- -y && \
    echo 'source $HOME/.cargo/env' >> ~/.bashrc && \
    /bin/bash -c "source $HOME/.cargo/env"

RUN echo 'export PATH="/root/.cargo/bin/:$PATH"' >> $HOME/.bashrc
RUN /bin/bash -c "source $HOME/.bashrc"

# Set default version
RUN /root/.cargo/bin/rustup install 1.66 && \
    /root/.cargo/bin/rustup default 1.66 && \
    /root/.cargo/bin/rustup target add wasm32-unknown-unknown

# RUN cargo install tezos-smart-rollup-installer --git https://gitlab.com/tezos/tezos
RUN /root/.cargo/bin/cargo install tezos-smart-rollup-installer

# CMD ["/bin/bash"]
