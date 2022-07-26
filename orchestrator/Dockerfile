FROM ubuntu:22.04

COPY ./target/debug/mergravity /usr/bin/mergravity

ENV RUST_LOG info,orchestrator=trace

WORKDIR /root

RUN /usr/bin/mergravity init

ENTRYPOINT ["/usr/bin/mergravity"]
STOPSIGNAL SIGTERM
