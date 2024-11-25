FROM caddy:alpine
WORKDIR /web
COPY ./dist /web/public
COPY ./Caddyfile /web/Caddyfile
CMD ["caddy", "run"]