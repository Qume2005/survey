FROM node:lts-alpine3.20 AS builder
WORKDIR /build
COPY . .
RUN npm i -g --dev pnpm -y
RUN pnpm install
RUN pnpm run build
FROM caddy:alpine
WORKDIR /web
COPY --from=builder /build/dist /web/public
COPY Caddyfile /web/Caddyfile
CMD ["caddy", "run", "--config", "/web/Caddyfile"]
