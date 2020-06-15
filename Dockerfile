FROM rust:1.44 AS build

COPY . /code
WORKDIR /code

RUN cargo test
RUN cargo build --release

FROM ubuntu:20.04

COPY --from=build /code/target/release/weave /bin/weave

ENTRYPOINT [ "/bin/weave" ]
CMD ["build"]
