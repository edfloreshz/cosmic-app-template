# COSMIC Application Template

A template for COSMIC applications.

## Getting Started

To get started, click the "Use this template" button above. This will create a new repository in your account with the contents of this template.

Once you have created a new repository from this template, you can clone it to your local machine and start developing your COSMIC application.

## Development

When you open the repository in your code editor, you will see a lot of comments in the code. These comments are there to help you get a basic understanding of what each part of the code does.

Once you feel comfortable with it, refer back to the [COSMIC documentation](https://pop-os.github.io/libcosmic/cosmic/) for more information on how to build COSMIC applications.

## Install

To install your COSMIC application, you will need [just](https://github.com/casey/just), if you're on Pop!\_OS, you can install it with the following command:

```sh
sudo apt install just
```

After you install it, you can run the following commands to build and install your application:

```sh
just build-release
sudo just install
```

To uninstall simply run

```sh
sudo just uninstall
```

## Publish

To create a `.deb` package you'll need to install `cargo-deb` if you haven't already.
```sh
cargo install cargo-deb
```

Then you can create a `.deb` package for your application with:
```sh
just debpkg
```

With cargo `cargo-deb` your package configuration is in `Cargo.toml` in the `[package.metadata.deb]` section.
Since TOML doesn't really support variables you have to update everything manually (for now).
