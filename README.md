# Encrypt Me

Shift bytes in a file.

For example, if the content of your file is "a", and you want to shift bytes by 2, the new content will be "c".

## How to use

You can test this program with the files that are in `test_files` directory.

### Encrypt

To encrypt you have to use `+`.

```sh
encrypt-me image.png + 10
```

### Decrypt

To decrypt use `-`.

```sh
encrypt-me image.png - 10
```

### Other uses

You can encrypt or decrypt multiple times the same file if you want.

## Install

### Build from source

Requirements:

- Rust compiler
- cargo

#### Local

1. Clone the repo and cd into it.
2. Build and install in cargo path.

```sh
cargo install --path .
```

#### crates.io

```sh
cargo install encrypt-me
```

### Download binary

You can download a github release for your OS and then move it into your binaries path (/usr/bin for linux and mac).

## Auto install

Use `install.sh` to auto install encryt-me for your OS.

This will download install script and run it.

```sh
curl -L https://raw.githubusercontent.com/RotrixLOL/encrypt-me/main/install.sh | sh
```

## TODO

- Finish `install.sh`
- Create github action to build and push binary.

## Contribute

You can contribute to this project if you want. The only rule is that you have to use [conventional commits](conventionalcommits.org) and common sense.
