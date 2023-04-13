FROM rust:1.68-bullseye as build
WORKDIR /app
COPY . .
RUN mv sqlx-data-sqlite.json sqlx-data.json
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && \
  apt-get upgrade -y && \
  apt-get install -y openssl ca-certificates && \
  rm -rf /var/lib/apt/lists/* && \
  useradd -ms /bin/bash ferris
USER ferris
WORKDIR /app
COPY --chown=ferris:ferris --from=build /app/target/release/rust-time-communication-hub /app/rust-time-communication-hub
EXPOSE 4501
EXPOSE 4500
CMD ["/app/rust-time-communication-hub"]
