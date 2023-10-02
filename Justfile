# List all available commands.
default:
	just --list

# Set up the project for development, installing required cargo tools and creating the database.
setup_dev:
	cargo install cargo-watch
	cargo install sqlx-cli --no-default-features --features sqlite,rustls,completions
	sqlx database setup

# Run the project in development mode, automatically restarting on changes.
watch_dev:
	cargo watch -q -c -w src/ -w .cargo/ -x "run"

# Run all the tests.
test_all:
	cargo test -- --nocapture

# Run a specific test.
test TEST:
	cargo test {{TEST}} -- --nocapture

# Run all the tests, automatically restarting on changes. Useful when developing the tests themselves.
watch_test_all:
	cargo watch -q -c -x "test -- --nocapture"

# Run a test, automatically restarting on changes. Useful when developing the tests themselves.
watch_test TEST:
	cargo watch -q -c -x "test {{TEST}} -- --nocapture"
