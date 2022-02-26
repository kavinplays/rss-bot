FROM rustlang/rust:nightly

WORKDIR /usr/src/rss-bot
COPY . .

RUN cargo install --path .

CMD ["rss-bot"]