.PHONY: build watch openocd run

build:
	cargo build

watch:
	cargo watch

openocd:
	openocd

run-blink:
	cargo run --example blink --target thumbv6m-none-eabi --features stm32f072
