FROM rust:bullseye AS build

COPY . .

RUN cargo build --release
#RUN cargo build

FROM debian:bullseye-slim

ARG VERSION=0.0.1

LABEL name="Redirector" author="Jan Harasym <jharasym@linux.com>" version="${VERSION}"

ARG PORT=8080
EXPOSE ${PORT}

COPY --from=build ["urls.toml", "/urls.toml", "target/release/url", "/", "entrypoint.sh", "/"]

ENTRYPOINT ["/entrypoint.sh"]
CMD []