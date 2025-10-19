# nanoservices/user-session-cache/cache-module/Dockerfile
FROM rust:latest as build
ENV PKG_CONFIG_ALLOW_CROSS=1

RUN apt-get update
RUN apt-get install libclang-dev -y

WORKDIR /app
COPY . .
RUN cargo build --release

FROM redis

COPY --from=build /app/target/release/libcache_module.so ./libcache_module.so


EXPOSE 6379
CMD ["redis-server", "--loadmodule", "./libcache_module.so"]
