FROM alpine:3
RUN apk --update add ca-certificates
COPY ./target/release/server /usr/local/bin/
USER 1001
ENTRYPOINT ["server"]
