# Unofficial Drunagor Randomizer

Use the tool [here](https://alexkazik.github.io/spawning-drunagor/).

## Translation

Currently the UI can't be translated and it's not that easy to translate the game data.

Though if you're interested in either one, please contact me
(via [email](mailto:spawning-drunagor+4575@tx0.eu) or [BGG](https://boardgamegeek.com/geekmail/compose?touser=txnull)).

## Running it yourself

### Requirements

- https://rustup.rs/
- `rustup target add wasm32-unknown-unknown`
- https://trunkrs.dev/#install

### Running

Run this application with the trunk development server:

```bash
trunk serve --open
```

### Building

```bash
trunk build --no-default-features
```

If the application will not be in the domain root (e.g. `https://example.com/spawning-drunagor`):

```bash
trunk build --no-default-features --release --public-url /spawning-drunagor
```
