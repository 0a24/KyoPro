# BUILD
cargo build --bin $1_$2$4 # abc301 a 1

# TEST
cat ./test/$1/$2/sample-$3.in | ./target/debug/$1_$2

