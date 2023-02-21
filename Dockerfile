FROM ubuntu
COPY ./target/release/server /usr/local/bin/
USER 1001
ENTRYPOINT ["server"]
