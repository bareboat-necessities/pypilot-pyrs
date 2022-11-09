#!/bin/bash -e

curl https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"

git clone https://github.com/konchunas/pyrs
cd pyrs/

cd ..
git clone https://github.com/pypilot/pypilot/
cd pyrs/

python3 ./pyrs.py /home/user/pypilot
