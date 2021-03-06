# wasm-rust-julia

See it in action [here](https://friarswood.net)

Client-side rendering of fractals in the browser using rust compiled to WebAssembly:

- dynamic Julia sets (the value of `c` is determined by the position of the mouse pointer)
- zoomable Mandelbrot set (left-click increases the magnification by 2 centred on the mouse pointer)

Build:

```bash
wasm-pack build
```

Serve:

```bash
cd www && npm run start
```

Based on [the tutorial here](https://rustwasm.github.io/docs/book/introduction.html)
