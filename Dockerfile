FROM alpine:3
COPY --from=certs /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY ./target/release/server /usr/local/bin/
USER 1001
ENTRYPOINT ["server"]
