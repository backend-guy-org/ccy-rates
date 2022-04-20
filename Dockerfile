FROM rust:1.60 as builder

WORKDIR /usr/src/app

COPY . .

RUN apt-get update && apt-get install -y cmake
RUN cargo install --path .

###############################################

FROM debian:buster-slim

RUN apt-get update && apt-get install -y ca-certificates tzdata
RUN rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/ccy-rates /usr/local/bin/ccy-rates

CMD ["ccy-rates"]