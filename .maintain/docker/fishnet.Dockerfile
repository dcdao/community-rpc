FROM ubuntu:22.04

COPY analysis/fishnet/target/release/cmta-fishnet /usr/local/bin

ENTRYPOINT [ "/usr/local/bin/cmta-fishnet" ]
