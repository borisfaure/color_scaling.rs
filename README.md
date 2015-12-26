color_scaling
=============

Functions to get a weighted color between 2 colors. For example, to generate a
gradient between 2 colors.

### Installation

This crate is fully compatible with Cargo. Just add it to your `Cargo.toml`:

```toml
[dependencies]
color_scaling = "*"
```

### Quick example

```rust
extern crate color_scaling;
extern crate image;

use color_scaling::scale_rgb;
use image::Rgb;

fn main() {
    let white     : Rgb<u8> = Rgb{ data: [255, 255, 255] };
    let orange    : Rgb<u8> = Rgb{ data: [255, 127,   0] };

    let light_orange = scale_rgb(&white, &orange, 0.8_f64);
    println!("light_orange={:?}", light_orange);
}
```

