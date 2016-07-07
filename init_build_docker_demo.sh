#!/bin/sh

cargo build --release

rm -rf build

mkdir -p build

git clone --depth 1 --branch master https://github.com/ivanceras/mockdata build/mockdata
git clone --depth 1 --branch master https://github.com/ivanceras/curtain-elm build/curtain-elm


. ./build/mockdata/local_db.sh

echo "localhost:5432:*:postgres:p0stgr3s" > build/.pgpass

sudo docker build -t curtain .
sudo docker run -p 8080:80 -p 3224:3224 -p 5433:5432 -t curtain



