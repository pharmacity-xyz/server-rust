FROM rust:1.59.0 AS builder

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM rust:1.59.0-slim AS runtime

WORKDIR /app
COPY --from=builder /app/target/release/pharmacity pharmacity
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENV STRIPE_SECRET_KEY sk_test_51Lt6NAGlVubd39S7721SdXDerAaaGPYD4oovoDhRKiXJASrObodzE9uUYavkk7ztBpgMdECUsjvNJSSA6jhD5wqA00CKYcuGlO
ENTRYPOINT ["./pharmacity"]
