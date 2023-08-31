OS := $(shell uname)

ifeq ($(OS), Darwin)
    # Set the URL for macOS version of SQLite
    SQLITE_ARCHIVE_NAME := sqlite-tools-osx-x86-3430000
else ifeq ($(OS), Linux)
    # Set the URL for Linux version of SQLite
    SQLITE_ARCHIVE_NAME := sqlite-tools-linux-x86-3430000
else
    $(error Unsupported operating system: $(OS))
endif

# Download and extract the latest SQLite binary to the 'tools' directory
tool_setup:
	mkdir -p tools
	curl -L https://www.sqlite.org/2023/$(SQLITE_ARCHIVE_NAME).zip -o tools/sqlite.zip
	unzip -j -o tools/sqlite.zip -d tools $(SQLITE_ARCHIVE_NAME)/sqlite3
	rm tools/sqlite.zip
	chmod +x tools/sqlite3
	@echo "SQLite 3.43.0 binary downloaded and extracted to the 'tools' directory."

cargo_devtool_setup:
	cargo install cargo-watch
	cargo install cargo-deploy
	@echo 

cargo_tool_setup:
	cargo install sqlx-cli
	@echo "Cargo tools installed."

project_database_setup:
	sqlx database create
	sqlx migrate run


dev_prerequisites:
	@echo "Installing dev prerequisites..."
	asdf 

dev:
	cargo watch -c -q -- cargo run 
