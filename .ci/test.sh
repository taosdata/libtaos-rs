#!/bin/bash
set -e
apt-get update
apt-get install -y wget curl build-essential openssl libssl-dev

# install server
wget https://www.taosdata.com/assets-download/TDengine-server-2.4.0.16-Linux-aarch64.tar.gz \
  && tar xvf TDengine-server-2.4.0.16-Linux-aarch64.tar.gz \
  && cd TDengine-server-2.4.0.16 \
  && ./install.sh -e no \
  && cd ../

nohup taosd &

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup-init

if [ -e .cargo/env ]; then
  echo use local .cargo
else
  export HOME=`pwd`
  bash rustup-init --profile default -y -v
  source $HOME/.cargo/env
fi

cargo test
cargo test --features rest,r2d2
