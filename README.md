# stegs

Generative SVG stegs for [stegano.graphics](https://stegano.graphics).

A steg is a small visual cipher: a string is encoded as a radial path, producing
spiral and fingerprint-like forms that can be rendered as SVG in Rust or in the browser
through WebAssembly.

## Status

This project is being revived for open-source release.

## What It Encodes

The encoder currently supports a compact alphabet:

```text
abcdefghijklmnopqrstuvwxyz ó
```

## Rust Usage

```rust
use stegs::{generate_spiral_svg, StegOptions};

fn main() {
    let options = StegOptions::default();
    let generated = generate_spiral_svg("mistrust authority", &options);

    std::fs::write("steg.svg", generated.svg).unwrap();
}
```

## Browser / WASM Usage

Build the WebAssembly package with:

```sh
wasm-pack build --target bundler --out-dir pkg
```

Then import it from a frontend bundler such as Vite:

```js
import init, { generate_steg_svg } from "stegs";

await init();
const svg = generate_steg_svg("all information should be free", "#f5c266", "transparent");
```

PNG export is best handled in the browser by drawing the generated SVG into a
canvas and saving the canvas as `image/png`.

## Development

```sh
cargo test
cargo fmt
wasm-pack test --node
```

## License

MIT. See `LICENSE` when the repository is prepared for public release.
