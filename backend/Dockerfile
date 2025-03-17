FROM rust:bookworm as build
COPY . /app
WORKDIR /app
RUN apt update && apt full-upgrade -y
RUN apt install -y cmake libclang-dev
RUN cargo build --release

FROM debian:bookworm
RUN apt update && apt full-upgrade -y
COPY --from=build /app/target/release/backend /app/backend
WORKDIR /app
RUN chmod +x backend
ENTRYPOINT [ "./backend" ]
