all:
	arm-none-eabi-objcopy target/thumbv8m.base-none-eabi/release/examples/blink target/thumbv8m.base-none-eabi/release/examples/blink.bin -O binary
	arm-none-eabi-objcopy target/thumbv8m.base-none-eabi/debug/examples/blink target/thumbv8m.base-none-eabi/debug/examples/blink.bin -O binary
