dev_prerequisites:
	@echo "Installing dev prerequisites..."
	asdf install
	cargo install cargo-watch
	cargo install sqlx-cli --no-default-features --features sqlite,rustls,completions
	sqlx database create

dev:
	cargo watch -c -q -- cargo run 
