# Rustshambo

An unbeatable rock-paper-scissor bot written in Rust with a UI made with Vue.js

You can play at: https://rock-paper-scissor.yashmehrotra.com

### Setup

```
# Creating a static binary to ship a smaller docker image
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-unknown-linux-gnu
```
