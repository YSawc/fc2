Toy nes emulator implemented with pure rust.

## dependencies

- SDL2

## Samples

- Show hello world!
``` rust
cargo run roms/hello-world.nes
```

- If wanna show sprites, use option of `show_sprites`

``` rust
cargo run show_sprites roms/hello-world.nes
```

Another roms locate roms directory.

## Tests

I use nestest for cpu tests.
``` rust
cargo run roms/nestest.nes
```

