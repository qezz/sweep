# Sweep

[![npm](https://img.shields.io/npm/v/swp)](https://www.npmjs.com/package/swp)
[![View on Crates.io](https://img.shields.io/crates/v/swp.svg)](https://crates.io/crates/swp)
[![Download](https://img.shields.io/badge/download-latest-informational.svg)](https://github.com/woubuc/sweep/releases/latest)
[![License](https://img.shields.io/github/license/woubuc/sweep.svg)](https://github.com/woubuc/sweep/blob/master/LICENSE)
[![Test Status](https://github.com/woubuc/sweep/workflows/tests/badge.svg)](https://github.com/woubuc/sweep/actions)

Sweep (`swp`) finds old projects that haven't been changed in more than a month. It will clean up and remove unnecessary directories containing libraries, dependencies, builds, etc. These files can easily be re-generated at any time by running install or build commands, and if you haven't worked on the project in a while you probably don't need them taking up space right now.

![Screenshot](readme_screenshot.png)

## Changes

This version of sweep works with a config file rather than with the
predefined set of rules.

Sample config:

```yaml

version: 1

entries:
  - name: "rust"
    trigger: "Cargo.toml"
    disposables: ["target"]
  - name: "npm"
    trigger: "package.json"
    disposables: ["node_modules", ".cache", "build", "dist"]
  - name: "java/pom"
    trigger: "pom.xml"
    disposables: ["target", ".gradle", "build"]
```

At the moment `config` is a required parameter. I'm looking forward to
using some default config somewhere in user local files
(e.g. `~/.config/` on linux)

## How to use

TL;DR:

```
cargo run -- --config sample-config.yml --all ~/
```

Yep, it's not very convenient so far.

For the original documentation check [this page](https://sweep.woubuc.be).

## Contributions

This project is a PoC. Please contribute to the original project.

## License
Sweep is published under the MIT license. See
the LICENSE file for more information.
