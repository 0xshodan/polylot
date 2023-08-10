FROM rust as build

WORKDIR /usr/polylot

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian11

COPY --from=build /usr/polylot/target/release/polylot /usr/local/bin/polylot

WORKDIR /usr/local/bin

CMD [ "polylot" ]
