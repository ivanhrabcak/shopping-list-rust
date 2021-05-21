RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN rustup default nightly
RUN rustup update
EXPOSE 8000
RUN cargo run

