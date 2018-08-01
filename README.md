# stm32f4-discovery-example
Rust STM32F4 discovery board example project

## Building

```bash
# rustc 1.29.0-nightly (54628c8ea 2018-07-30)
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
- [example F3 HAL crate](https://github.com/japaric/stm32f30x-hal)
- [example F3 board support crate](https://github.com/japaric/f3)
- [my F4 HAL crate](https://github.com/jonlamb-gh/stm32f407-hal)
- [my F4 board support crate](https://github.com/jonlamb-gh/f4-bsp)
