# stm32f4-discovery-example
Rust STM32F4 discovery board example project

## Building

```bash
cargo build
```

## Debugging

```bash
openocd -f interface/stlink-v2-1.cfg -f target/stm32f4x.cfg
```

```bash
cargo run

# or manually
# arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/stm32f4-discovery-example
```

## Links

- [cortex-m quickstart](http://blog.japaric.io/quickstart/)
- [device crate](https://github.com/adamgreig/stm32-rs/tree/master/stm32f4)
