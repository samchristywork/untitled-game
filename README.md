![Banner](https://s-christy.com/status-banner-service/untitled-game/banner-slim.svg)

## Overview

This is an experimental game that I have been working on. I have been using it
to learn webassembly and have been playing around with some game-making concepts
like entity-component systems and graphics.

## Features

## Dependencies

You will need Rust, Cargo, the Rust webassembly target architecture, and WASM
Pack. You can get the latter two with these commands:

```
cargo install wasm-pack
rustup target add wasm32-unknown-unknown
```

## Usage

Build the code and place resultant WASM output in `public/pkg/`.

```
wasm-pack build --target web --out-dir public/pkg/
```

Use any server to host the static files in `public`. For instance, you could run
`python3 -m http.server` from the `public` directory to serve the files, or you
could use any other web server.

## License

This work is licensed under the GNU General Public License version 3 (GPLv3).

[<img src="https://s-christy.com/status-banner-service/GPLv3_Logo.svg" width="150" />](https://www.gnu.org/licenses/gpl-3.0.en.html)
