# Try in Docker

If you prefer to try it before installing, here's the snippet for your
convenience.

```bash
docker run -w / -it --rm ubuntu sh -uec '
  apt-get update -y
  apt-get install -y wget tar vim less
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

One way to keep an eye on the releases is to [watch the repository][4].

## Community Maintained Repositories

xplr can be installed from one of the following community maintained
repositories:

[![packaging status][5]][6]

### Cross-platform

#### [Nixpkgs][10]

```
nix-env -f https://github.com/NixOS/nixpkgs/tarball/master -iA xplr
```

Or

```nix
# configuration.nix or darwin-configuration.nix
environment.systemPackages = with nixpkgs; [
  xplr
  # ...
];
```

#### [Home Manager][30]

```nix
# home.nix
home.packages = with nixpkgs; [
  xplr
  # ...
];
```

Or

```nix
# home.nix
programs.xplr = {
  enable = true;

  # Optional params:
  plugins = {
    tree-view = fetchFromGitHub {
      owner = "sayanarijit";
      repo = "tree-view.xplr";
    };
    local-plugin = "/home/user/.config/xplr/plugins/local-plugin";
  };
  extraConfig = ''
    require("tree-view").setup()
    require("local-plugin").setup()
  '';
};
```

### Arch Linux

(same for Manjaro Linux)

#### [Official Community Repo][7]

```
sudo pacman -S xplr
```

#### [AUR][8]

Git version:

```
paru -S xplr-git
```

### Alpine Linux

#### [Edge Testing Repo][27]

```
# Add the following line in /etc/apk/repositories:
# https://dl-cdn.alpinelinux.org/alpine/edge/testing

apk add xplr bash less
```

### Void Linux

#### [void-templates by shubham][9]

### Gentoo

#### [Overlay GURU][28]

### macOS

Make sure you have the latest version of [GNU core utilities][29] installed.

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

- [GNU/Linux][16]
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

[1]: #direct-download
[2]: #from-cratesio
[3]: #build-from-source
[4]: https://github.com/sayanarijit/xplr/watchers
[5]: https://repology.org/badge/vertical-allrepos/xplr.svg
[6]: https://repology.org/project/xplr/versions
[7]: https://archlinux.org/packages/extra/x86_64/xplr
[8]: https://aur.archlinux.org/packages/?O=0&SeB=n&K=xplr&outdated=&SB=n&SO=a&PP=50&do_Search=Go
[9]: https://github.com/shubham-cpp/void-pkg-templates
[10]: https://github.com/NixOS/nixpkgs/blob/master/pkgs/by-name/xp/xplr/package.nix
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
[23]: https://github.com/sayanarijit/xplr/assets/11632726/3b61e8c8-76f0-48e8-8734-50e9e7e495b7
[25]: https://gifyu.com/image/tF2D
[26]: https://github.com/sayanarijit/xplr/releases/latest/download/xplr-linux-musl.tar.gz
[27]: https://pkgs.alpinelinux.org/packages?name=xplr
[28]: https://gpo.zugaina.org/Overlays/guru/app-misc/xplr
[29]: https://formulae.brew.sh/formula/coreutils
[30]: https://github.com/nix-community/home-manager/blob/master/modules/programs/xplr.nix
