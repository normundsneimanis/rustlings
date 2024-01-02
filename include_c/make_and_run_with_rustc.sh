#!/bin/sh
gcc -c -fpic -Iinclude src/test.c -shared -o libtest.so
sudo cp libtest.so /usr/lib/
bindgen include/test.h -o src/bindings.rs
rustc -L. -ltest src/main.rs
./main
