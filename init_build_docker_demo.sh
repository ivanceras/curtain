#!/bin/sh

cargo build --release

rm -rf build

mkdir -p build

git clone --depth 1 --branch master https://github.com/ivanceras/mockdata build/mockdata
git clone --depth 1 --branch master https://github.com/ivanceras/curtain-elm build/curtain-elm


. ./build/mockdata/local_db.sh

echo "$HOST:$PORT:*:$USER:$PASSWORD" > build/.pgpass

if [ $USER = "postgres" ]; then
    echo "ALTER USER $USER WITH PASSWORD '$PASSWORD';" > build/setup_postgres_user.sql
else
    echo "CREATE USER $USER WITH SUPERUSER PASSWORD '$PASSWORD';" > build/setup_postgres_user.sql
fi

echo "CREATE DATABASE $DB WITH OWNER $USER;" >> build/setup_postgres_user.sql


docker build -t curtain .
docker run -p 8080:80 -p 3224:3224 -t curtain



