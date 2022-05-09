gcc extern.c -o extern -lextern_fn -L./target/release

LD_LIBRARY_PATH=./target/debug ./call_rust

