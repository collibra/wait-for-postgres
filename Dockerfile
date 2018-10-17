FROM rust

RUN rustup target install x86_64-unknown-linux-musl
VOLUME /usr/local/cargo/registry/

ADD Cargo.toml src /src/
WORKDIR /src

CMD cargo build --target x86_64-unknown-linux-musl --release \
	&& strip target/x86_64-unknown-linux-musl/release/wait-for-postgres
