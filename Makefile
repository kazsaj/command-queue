build:
	cargo build

build-release:
	cargo build --release

test:
	cargo test

run:
	docker-compose up -d && cargo run alfa bravo charlie

redis-cli:
	docker-compose up -d && docker exec -it command_queue_redis redis-cli

format:
	cargo fmt