# Build

See [install.md](./docs/en/src/install.md#build-from-source)

Note: xplr ships with vendored luajit. If the platform can't compile this,
you need to compile using `--no-default-features` argument to avoid using
vendored luajit, so that you can static link luajit yourself.

# Release

The final binary `target/release/xplr` can be shipped with the following assets

- [License](./LICENSE)
- [Desktop Entry](./assets/desktop/xplr.desktop)
- [Desktop Icons](./assets/icon/)
- [Offline Docs](./docs/en/src)
- [Lua Configuration Example](./src/init.lua)
