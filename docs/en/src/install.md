# Try in Docker

If you prefer to try it before installing, here's the snippet for your
convenience.

```bash
docker run -w / -it --rm ubuntu sh -uec '
  apt-get update -y
  apt-get install -y wget tar vim
  wget https://github.com/sayanarijit/xplr/releases/latest/download/xplr-linux.tar.gz
  tar -xzvf xplr-linux.tar.gz
  ./xplr
'
```

# Install

You can install xplr using one of the following ways. Each has their own
advantages and limitations.

For example, the [Direct Download][1], [From crates.io][2], and
[Build From Source][3] methods allow the users to install the latest possible
version of xplr, but they have one common drawback - the user will need to keep
an eye on the releases, and manually upgrade xplr when a new version is
available.

One way to keep an eye on the releases is to
[watch the repository][4].

## Community Maintained Repositories

xplr can be installed from one of the following community maintained
repositories:

[![packaging status][5]][6]

### Arch Linux

#### [Official Community Repo][7]

```
sudo pacman -S xplr
```

#### [AUR][8]

Binary version:

```
paru -S xplr-bin
```

Git version:

```
paru -S xplr-git
```

### Void Linux

#### [void-templates by shubham][9]

### Nix(OS)

#### [Nixpkgs][10]

```
nix-env -f https://github.com/NixOS/nixpkgs/tarball/master -iA xplr
```

### macOS

#### [MacPorts][11]

```
sudo port selfupdate
sudo port install xplr
```

#### [Homebrew][12]

Stable branch:

```
brew install xplr
```

HEAD branch:

```
brew install --head xplr
```

### FreeBSD

#### [ports][13]

```
pkg install xplr
```

Or

```
cd /usr/ports/misc/xplr
make install clean
```

### NetBSD

#### [pkgsrc][14]

```
pkgin install xplr
```

Or

```
cd /usr/pkgsrc/sysutils/xplr
make install
```

## Direct Download

One can directly download the standalone binary from the
[releases][15].

Currently, the following options are available for direct download:

- [Linux][16]
- [macOS][17]

Command-line instructions:

```bash
platform="linux"  # or "macos"

# Download
wget https://github.com/sayanarijit/xplr/releases/latest/download/xplr-$platform.tar.gz

# Extract
tar xzvf xplr-$platform.tar.gz

# Place in $PATH
sudo mv xplr /usr/local/bin/
```

## From [crates.io][18]

Prerequisites:

- [Rust toolchain][19],
- [gcc][20]
- [make][21]

Command-line instructions:

```bash
cargo install --locked --force xplr
```

## Build From Source

Prerequisites:

- [git][22]
- [Rust toolchain][19]
- [gcc][20]
- [make][21]

Command-line instructions:

```bash
# Clone the repository
git clone https://github.com/sayanarijit/xplr.git
cd xplr

# Build
cargo build --locked --release --bin xplr

# Place in $PATH
sudo cp target/release/xplr /usr/local/bin/
```

## Android

### [Termux][23]

[![xplr-termuxfd3c398d3cf4bcbc.md.jpg][24]][25]

> Please note that xplr isn't heavily tested on Termux, hence things might
> need a little tweaking and fixing for a smooth usage experience.

- Install build dependencies

  ```bash
  pkg install rustc cargo make
  ```

- Install `xplr`

  ```bash
  cargo install --locked --force xplr
  ```

- Setup storage

  ```bash
  termux-setup-storage
  ```

- Setup config and runtime dir

  ```bash
  export XDG_CONFIG_HOME="$PWD/storage/.config"
  export XDG_RUNTIME_DIR="$PWD/storage/run"

  mkdir -p "$XDG_CONFIG_HOME" "$XDG_RUNTIME_DIR"
  ```

- Run
  ```bash
  ~/.cargo/bin/xplr
  ```

[1]: #direct-download
[2]: #from-a-hrefhttpscratesiocratesxplrcratesioa
[3]: #build-from-source
[4]: https://github.com/sayanarijit/xplr/watchers
[5]: https://repology.org/badge/vertical-allrepos/xplr.svg
[6]: https://repology.org/project/xplr/versions
[7]: https://archlinux.org/packages/community/x86_64/xplr
[8]: https://aur.archlinux.org/packages/?O=0&SeB=n&K=xplr&outdated=&SB=n&SO=a&PP=50&do_Search=Go
[9]: https://github.com/shubham-cpp/void-pkg-templates
[10]: https://github.com/NixOS/nixpkgs/blob/master/pkgs/applications/misc/xplr
[11]: https://ports.macports.org/port/xplr
[12]: https://formulae.brew.sh/formula/xplr
[13]: https://cgit.freebsd.org/ports/plain/misc/xplr/
[14]: https://pkgsrc.se/sysutils/xplr
[15]: https://github.com/sayanarijit/xplr/releases
[16]: https://github.com/sayanarijit/xplr/releases/latest/download/xplr-linux.tar.gz
[17]: https://github.com/sayanarijit/xplr/releases/latest/download/xplr-macos.tar.gz
[18]: https://crates.io/crates/xplr
[19]: https://www.rust-lang.org/tools/install
[20]: https://gcc.gnu.org/
[21]: https://www.gnu.org/software/make/
[22]: https://git-scm.com/
[23]: https://termux.com/
[24]: https://s3.gifyu.com/images/xplr-termuxfd3c398d3cf4bcbc.md.jpg
[25]: https://gifyu.com/image/tF2D
