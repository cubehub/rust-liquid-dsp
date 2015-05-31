# rust-liquid-dsp
Rust API for [liquid-dsp](https://github.com/jgaeddert/liquid-dsp).

## dependencies
### [liquid-dsp](https://github.com/jgaeddert/liquid-dsp)
    git clone git://github.com/jgaeddert/liquid-dsp.git
    cd liquid-dsp
    ./bootstrap.sh
    ./configure
    make
    sudo make install

## usage
Put this in your `Cargo.toml`:

```toml
[dependencies.liquid_dsp]
git = "https://github.com/cubehub/rust-liquid-dsp.git"
```

And this in your crate root:

```rust
extern crate liquid_dsp;
```
