![Banner](https://s-christy.com/status-banner-service/untitled-game/banner-slim.svg)

## Overview

This is an experimental game that I have been working on. I have been using it
to learn webassembly and have been playing around with some game-making concepts
like entity-component systems and graphics.

## Game Concepts

One of the main goals of this game is to play around with the idea that
everything in the game is an entity with attributes that modify the thing's
behavior. An example of an attribute is "harmful". When such an entity comes
into contact with the player character, the player's health is reduced. Another
example is "fast", which is an attribute that modifies the speed of an entity.
Using this simple principle, we can rapidly code complex behavior into the game.

## Features

- Attribute and effects systems.
- Demonstrates complex game logic from simple ECS principles.
- Sprite transforms including flip, rotate, scale, and translate.
- Multiple maps/scrollable screens.
- Function hooks can be called when entities are "activated".
- Collision detection between entities.
- Demonstrates a bi-directional interface between Rust and Javascript.
- Game loop using request animation frame.
- Visuals like sprites, text, and HUD-style UI elements.
- Debug mode and statusline.
- Keyboard control including modifier keys like shift and control.
- A lovingly crafted sprite sheet and logic to display the sprites.

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
