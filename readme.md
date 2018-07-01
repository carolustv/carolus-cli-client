# Carolus CLI

Quick start:

```bash
export CAROLUS_SERVER_URL=http://carolus-host:8000
cargo run --release -- play movie -t '<name of movie>'
cargo run --release -- play tv -t '<name of movie>' -s 1 -e 1
```

## Completions

Supports completions for multiple shells

Eg. if you have `oh-my-zsh`:

```bash
carolus-cli completions zsh > ~/.oh-my-zsh/completions/_carolus-cli
```

## GStreamer

This program uses GStreamer for the video viewer so you need
to have the appropriate codecs installed for whatever videos
you are trying to play.

* [Arch](https://wiki.archlinux.org/index.php/GStreamer)

## License

This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at [Mozilla MPL 2.0](http://mozilla.org/MPL/2.0/).
