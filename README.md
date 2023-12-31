NES emulator implemented with pure rust.

## dependencies

- SDL2
If using ubuntu, run above to install.
```
sudo add-apt-repository -y "deb http://archive.ubuntu.com/ubuntu `lsb_release -sc` main universe restricted multiverse"
sudo apt-get update -y -qq
sudo apt-get install libsdl2-dev
sudo apt-get update -y -qq
```

## Samples

- hello world!
``` rust
cargo run roms/hello-world.nes
```

- If show sprites, use `show_sprites` as option.

``` rust
cargo run show_sprites roms/hello-world.nes
```

- Tests CPU operations include unformula.
``` rust
cargo run roms/nestest.nes
```

- Tests CPU operations and dump cpu info without gui rendering.
```
cargo run --features nestest_without_gui,with_dump roms/nestest.nes
```

Another roms locate roms directory.

## Benches
Measuring performance with [criterion.rs](https://github.com/bheisler/criterion.rs) helps easy improvements.
To show benches, run cargo bench and show generated HTML files.
```
cargo bench
open ./target/criterion/report/index.html
```

## Tips

- Poor rendering performance will be improved when build.
``` rust
cargo run --release roms/nestest.nes
```

- F1: Save state. After saved, save file locates saves/[rom_name]_save.json.
- F2: Load file. Load save file named from loaded rom file locates saves.
