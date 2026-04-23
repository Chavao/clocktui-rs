.PHONY: build install release clean uninstall

BINARY_NAME := clocktui
CARGO ?= cargo
BINDIR ?= $(HOME)/bin
CONFIG_DIR ?= $(HOME)/.config/clocktui
THEME_SOURCE_DIR ?= themes
THEME_INSTALL_DIR ?= $(CONFIG_DIR)/themes
CONFIG_SOURCE ?= config/default/config.toml
CONFIG_FILE ?= $(CONFIG_DIR)/config.toml

build:
	@$(CARGO) build --release

install: build
	@mkdir -p $(BINDIR) $(THEME_INSTALL_DIR)
	@install -m 755 target/release/$(BINARY_NAME) $(BINDIR)/$(BINARY_NAME)
	@find "$(THEME_SOURCE_DIR)" -mindepth 1 -maxdepth 1 -type d | while read -r theme_dir; do \
		theme_name=$$(basename "$$theme_dir"); \
		mkdir -p "$(THEME_INSTALL_DIR)/$$theme_name"; \
		cp -R "$$theme_dir"/. "$(THEME_INSTALL_DIR)/$$theme_name"/; \
	done
	@if [ ! -f "$(CONFIG_FILE)" ]; then install -m 644 $(CONFIG_SOURCE) "$(CONFIG_FILE)"; fi

release: install

clean:
	@$(CARGO) clean

uninstall:
	@rm -f $(BINDIR)/$(BINARY_NAME)
