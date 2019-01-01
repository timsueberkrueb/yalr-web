#!/bin/bash

set -ex

mkdir -p build/

if [[ ! -d "build/yalr" ]] ; then
  git clone https://github.com/timsueberkrueb/yalr.git build/yalr;
fi

cd build/yalr && \
  git checkout develop && \
  git pull && \
  cd ../..
