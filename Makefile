build:
	cargo build

build-release:
	cargo build --release

build-alpine-release:
	rustup target add x86_64-unknown-linux-musl && cargo build --release --frozen --target x86_64-unknown-linux-musl

test:
	cargo test

run:
	docker-compose up -d && COMMAND_QUEUE_LOG_LEVEL=DEBUG cargo run alfa alfa bravo charlie

redis-cli:
	docker-compose up -d && docker exec -it command_queue_redis redis-cli

format:
	cargo fmt