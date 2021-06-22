Install
=======

You can install xplr using one of the following ways. Each has their own
advantages and limitations.

For example, the [Direct Download](#direct-download),
[From crates.io](#from-a-hrefhttpscratesiocratesxplrcratesioa), and
[Build From Source](#build-from-source) methods allow the users to install the
latest possible version of xplr, but they have one common drawback - the user
will need to keep an eye on the releases, and manually upgrade xplr when a new
version is available.

One way to keep an eye of the releases is to
[watch the repository](https://github.com/sayanarijit/xplr/watchers).


Community Maintained Repositories
---------------------------------

xplr can be installed from one of the following community maintained
repositories:

[![packaging status](https://repology.org/badge/vertical-allrepos/xplr.svg)](https://repology.org/project/xplr/versions)


### Arch Linux

#### [Official Community Repo](https://archlinux.org/packages/community/x86_64/xplr)

```
sudo pacman -Syu xplr
```

#### [AUR](https://aur.archlinux.org/packages/?O=0&SeB=n&K=xplr&outdated=&SB=n&SO=a&PP=50&do_Search=Go)

Binary version:

```
paru -S xplr-bin
```

Git version:

```
paru -S xplr-git  # git version
```

### Void Linux

#### [void-templates by shubham](https://github.com/shubham-cpp/void-pkg-templates)

### Nix(OS)

#### [Nixpkgs](https://github.com/NixOS/nixpkgs/blob/master/pkgs/applications/misc/xplr)

```
nix-env -f https://github.com/NixOS/nixpkgs/tarball/master -iA xplr
```

### macOS

#### [MacPorts](https://ports.macports.org/port/xplr)

```
sudo port selfupdate
sudo port install xplr
```

#### [Homebrew](https://formulae.brew.sh/formula/xplr)

Stable branch:

```
brew install xplr
```

HEAD branch:

```
brew install --head xplr
```

### FreeBSD

#### [ports](https://cgit.freebsd.org/ports/plain/misc/xplr/)

```
cd /usr/ports/misc/xplr
make install
```

### NetBSD

#### [pkgsrc](https://pkgsrc.se/sysutils/xplr)

```
pkgin install xplr
```

Or build from source

```
cd /usr/pkgsrc/sysutils/xplr
make install
```


Direct Download
---------------

One can directly download the standalone binary from the
[releases](https://github.com/sayanarijit/xplr/releases).

Currently, the following options are available for direct download:

- [Linux](https://github.com/sayanarijit/xplr/releases/latest/download/xplr-linux.tar.gz)
- [macOS](https://github.com/sayanarijit/xplr/releases/latest/download/xplr-macos.tar.gz)

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


From [crates.io](https://crates.io/crates/xplr)
-----------------------------------------------

Prerequisites:

- [Rust toolchain](https://www.rust-lang.org/tools/install),
- [gcc](https://gcc.gnu.org/)
- [make](https://www.gnu.org/software/make/)

Command-line instructions:

```bash
cargo install --force xplr
```


Build From Source
-----------------

Prerequisites:

- [git](https://git-scm.com/)
- [Rust toolchain](https://www.rust-lang.org/tools/install)
- [gcc](https://gcc.gnu.org/)
- [make](https://www.gnu.org/software/make/)

Command-line instructions:

```bash
# Clone the repository
git clone https://github.com/sayanarijit/xplr.git
cd xplr

# Build
cargo build --release --bin xplr

# Place in $PATH
sudo cp target/release/xplr /usr/local/bin/
```


Android
-------

### [Termux](https://termux.com/)

[![xplr-termuxfd3c398d3cf4bcbc.md.jpg](https://s3.gifyu.com/images/xplr-termuxfd3c398d3cf4bcbc.md.jpg)](https://gifyu.com/image/tF2D)

> Please note that xplr isn't heavily tested on Termux, hence things might
> need a little tweaking and fixing for a smooth usage experience.

- Install build dependencies

  ```bash
  pkg install rustc cargo make
  ```

- Install `xplr`

  ```bash
  cargo install --force xplr
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
