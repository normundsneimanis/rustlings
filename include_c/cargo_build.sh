#!/bin/sh
gcc -c -fpic -Iinclude src/test.c -shared -o libtest.so
sudo cp libtest.so /usr/lib/
cargo build

