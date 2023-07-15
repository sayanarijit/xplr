# Build

See [install.md](./docs/en/src/install.md#build-from-source)

Note: xplr ships with vendored lua54. If the platform can't compile this,
you need to grep out the feature "vendored" from the "mlua" dependency
specified in [Cargo.toml](./Cargo.toml), and static link luajit yourself.

# Release

The final binary `target/release/xplr` can be shipped with the following assets

- [License](./LICENSE)
- [Desktop Entry](./assets/desktop/xplr.desktop)
- [Desktop Icons](./assets/icon/)
- [Offline Docs](./docs/en/src)
- [Lua Configuration Example](./src/init.lua)
