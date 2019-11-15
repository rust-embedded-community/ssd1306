#! /bin/sh
docker run \
  --volume $PWD:/home/cross/project \
  --volume $HOME/.cargo/registry:/home/cross/.cargo/registry \
    ragnaroek/rust-raspberry:1.39.0 \
    build --release
