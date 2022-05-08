FROM gcr.io/distroless/cc

USER nobody
ENV LOG=1

COPY ./target/release/zero2prod /zero2prod

ENTRYPOINT [ "/zero2prod" ]