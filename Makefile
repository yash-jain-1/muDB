CARGO ?= cargo
BINARY ?= mudb

.PHONY: all build run clean

all: build

build:
	$(CARGO) build --release

run:
	$(CARGO) run

clean:
	$(CARGO) clean
