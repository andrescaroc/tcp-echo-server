FROM alpine:3 as certs
RUN apk --update add ca-certificates

FROM scratch
COPY --from=certs /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY ./target/release/server /usr/local/bin/
USER 1001
ENTRYPOINT ["server"]
