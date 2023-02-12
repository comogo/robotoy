# Controller

It uses SDL2 to get input from the joystick.


## C version


### Compiling

```bash
cd c
gcc main.c -o build/controller -lSDL2
```

### Running

```bash
cd c
./build/controller
```

## Rust version

### Dependencies

- [SDL2](https://github.com/Rust-SDL2/rust-sdl2)
- [nrf24l01](https://github.com/rtxm/rust-nrf24l01)

### Compiling

```bash
cd rust
cargo build
```

### Running

```bash
cd rust
cargo run
```
