# Controller

It uses SDL2 to get input from the joystick and sends it to the robot through the NRF24L01+ module.

## Protocol

The protocol has a payload of 13 bytes.

|Button|Payload byte index| Bit index|Data type|Size|Range|Description|
|---|:---:|:---:|:---|:---:|:---:|:---|
|Cross|0|0|Bit|1|0\|1|
|Circle|0|1|Bit|1|0\|1|
|Square|0|2|Bit|1|0\|1|
|Tringagle|0|3|Bit|1|0\|1|
|Select|0|4|Bit|1|0\|1|
|Start|0|5|Bit|1|0\|1|
|L1|0|6|Bit|1|0\|1|
|R1|0|7|Bit|1|0\|1|
|L2|1-2|-|unsigned int16|2 Bytes|0 ... 32_768| 1: MSB </br> 2: LSB|
|R2|3-4|-|unsigned int16|2 Bytes|0 ... 32_768| 3: MSB </br> 4: LSB|
|Yaw|5-6|-|signed int16|2 Bytes|-32_768 ... 32_767 | 5: MSB </br> 6: LSB|
|Throttle|7-8|-|signed int16|2 Bytes|-32_768 ... 32_767 | 7: MSB </br> 8: LSB|
|Pitch|9-10|-|signed int16|2 Bytes|-32_768 ... 32_767 | 9: MSB </br> 10: LSB|
|Roll|11-12|-|signed int16|2 Bytes|-32_768 ... 32_767 | 11: MSB </br> 12: LSB|

## Dependencies

- [SDL2](https://github.com/Rust-SDL2/rust-sdl2)

## Compiling

```bash
cd rust
cargo build
```

## Running

```bash
cd rust
cargo run
```
