.PHONY: build install release clean uninstall

BINARY_NAME := clocktui
CARGO ?= cargo
BINDIR ?= $(HOME)/bin

build:
	@$(CARGO) build --release

install: build
	@mkdir -p $(BINDIR)
	@install -m 755 target/release/$(BINARY_NAME) $(BINDIR)/$(BINARY_NAME)

release: install

clean:
	@$(CARGO) clean

uninstall:
	@rm -f $(BINDIR)/$(BINARY_NAME)
