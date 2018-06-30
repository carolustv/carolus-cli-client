# Carolus CLI

Quick start:

```bash
export CAROLUS_SERVER_URL=http://localhost:8000
cargo run --release -- play movie -t '<name of movie>'
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
