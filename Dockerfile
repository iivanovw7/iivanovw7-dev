FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

WORKDIR /app

# stage 1, prepare the recipe for build caching
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# stage 2, copy over source code and build
FROM chef AS rust_builder
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release

# stage 2b, build our css as we don't have a formal preprocessor
FROM node:bookworm-slim as node_builder

WORKDIR /app

RUN corepack enable

# copy on over all the dependencies
COPY styles ./styles
COPY assets ./assets
COPY package.json .
COPY esbuild.config.css.mjs .
COPY .env .
COPY .nvmrc .
COPY config.toml .

COPY ./templates ./templates

RUN pnpm install;
RUN pnpm run build:css 
RUN npm install pm2 -g

FROM debian:bookworm-slim AS runtime

WORKDIR /app

COPY --from=rust_builder /app/target/release/iivanovw7-dev ./server
COPY --from=node_builder /app/assets ./assets
COPY --from=node_builder /app/node_modules ./node_modules
COPY --from=node_builder /app/.env .
COPY --from=node_builder /app/config.toml .

EXPOSE 80
EXPOSE 443
EXPOSE 8080

ENTRYPOINT ["/app/server"]
