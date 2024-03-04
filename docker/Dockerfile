FROM ubuntu:22.04

RUN apt-get update && \
    apt-get install -y curl git zip build-essential && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh && \
    chmod +x rustup.sh && \
    ./rustup.sh -y && \
    bash -lc "source $HOME/.cargo/env && rustup default nightly" && \
    mkdir -p /build-arena

ENV HOME=/root

COPY ./ubuntu_builder.sh /
RUN chmod +x /ubuntu_builder.sh

WORKDIR /build-arena
CMD /ubuntu_builder.sh
