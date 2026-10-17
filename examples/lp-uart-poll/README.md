# lp-uart-poll

Bare-metal UART output example for the **E902 (LP) core** of the BL808 SoC.

Demonstrates basic `UART0` initialization and blocking transmission using a polled TX FIFO status.

---

## Features

- Uses the `bl808-pac` with the `lp` feature.
- Implements `core::fmt::Write` for the UART peripheral.
- Polling-based transmission (no interrupts required).

---

## Building

```sh
cargo build --release -p lp-uart-poll
```
