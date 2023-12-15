cargo install cbindgen

cd rust-borrows-c-string/
cargo build
cbindgen --config cbindgen.toml --crate rust-borrows-c-string --output rust-borrows-c-string.h
cd ..
g++ -g -O0 -fsanitize=address main.cpp rust-borrows-c-string/target/debug/librust_borrows_c_string.so -o borrow
./borrow

