dev_prerequisites:
	@echo "Installing dev prerequisites..."
	asdf install
	cargo install cargo-watch
	cargo install cargo-deploy
	cargo install sqlx-cli --no-default-features --features sqlite,rustls,completions
	sqlx database create

dev:
	cargo watch -c -q -- cargo run 

dev_test:
	cargo watch -c -q -- cargo test
