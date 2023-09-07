# List all available commands.
default:
	just --list

# Set up the project for development, installing required cargo tools and creating the database.
setup_dev:
	cargo install cargo-watch
	cargo install cargo-deploy
	cargo install sqlx-cli --no-default-features --features sqlite,rustls,completions
	sqlx database create

# Run the project in development mode, automatically restarting on changes.
watch_dev:
	cargo watch -q -c -w src/ -w .cargo/ -x "run"

# Run the tests, automatically restarting on changes. Useful when developing the tests themselves.
watch_dev_test:
	cargo watch -q -c -x "test -- --nocapture"
