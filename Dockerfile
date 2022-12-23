FROM ubuntu as build1

RUN apt-get -qq update

RUN apt-get install -y -q \
    build-essential \
    curl

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

FROM build1

WORKDIR /recipe/
COPY . .
RUN cargo build --release
CMD ["./target/release/recipe"]
