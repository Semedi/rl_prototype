# rl_prototype

The project aims to have a base template for building roguelike games with Amethyst Game Engine.

Some of the provided features are:

- Project organization
- Spritesheet Render
- Tilemap Loaded
- Basic transiction between Roguelike states: [Input - PlayerTurn - AI]
- Helpers functions



## How to run

To run the game, run the following command, which defaults to the `vulkan` graphics backend:

```bash
cargo run
```

Windows and Linux users may explicitly choose `"vulkan"` with the following command:

```bash
cargo run --no-default-features --features "vulkan"
```




## Tiles

* https://www.kenney.nl/assets/bit-pack
