FROM rust:1.75 as builder
WORKDIR /usr/src/myapp

COPY . .

ADD ./infrastructure/certs/crates.io.crt /usr/local/share/ca-certificates/crates.io.crt
ADD ./infrastructure/certs/static.crates.io.crt /usr/local/share/ca-certificates/static.crates.io.crt

RUN apt install ca-certificates && \
    update-ca-certificates

RUN chmod 644 /usr/local/share/ca-certificates/crates.io.crt \
    && chmod 644 /usr/local/share/ca-certificates/static.crates.io.crt && update-ca-certificates

RUN cargo install --path .

CMD ["webserver"]
