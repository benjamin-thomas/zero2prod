# To debug:
#   - FROM gcr.io/distroless/cc:debug
#   - docker run -u root --entrypoint=/busybox/sh --rm -it local/zero2prod
FROM gcr.io/distroless/cc

USER nobody
ENV LOG=1

EXPOSE 8080

USER nonroot:nonroot
COPY ./target/release/zero2prod /zero2prod

ENTRYPOINT [ "/zero2prod" ]