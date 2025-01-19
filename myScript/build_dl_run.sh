#! /bin/bash

# BUILD
cargo build --bin $1_$2 


# DOWNLOAD TEST
if [ ! -e test/$1/$2 ]; then
    oj dl -d test/$1/$2 https://atcoder.jp/contests/$1/tasks/$1_$2
fi


# RUN
oj test -c "./target/debug/$1_$2" -d test/$1/$2
