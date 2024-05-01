# Contributing guide

## Setup

In order to execute development tasks, you need to install [cargo-make](https://github.com/sagiegurari/cargo-make).

## Lint

```sh
cargo make lint
```

## Format

```sh
cargo make format
```

## Release

1. Update *CHANGELOG.md* and bump version.
1. Push the changes to the main branch.
1. Create and push a GIT tag from latest main branch:

```sh
git tag v0.1.0
git push upstream v0.1.0
```

> [!NOTE]
> Change `v0.1.0` to the new version number.
