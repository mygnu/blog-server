FROM ubuntu:22.04

# environment variables
ENV RUST_LOG=info
ENV PORT=3000
ENV DB_PATH=/config/blog.sqlite3



RUN set -ex; \
    \
    apt-get update; \
    apt-get upgrade -y; \
    apt-get install -y --no-install-recommends \
    ca-certificates libsqlite3-dev \
    ; \
    rm -rf /var/lib/apt/lists/* \
    ;

COPY target/release/blog-server /usr/local/bin/blog-server


CMD [ "/usr/local/bin/blog-server" ]
