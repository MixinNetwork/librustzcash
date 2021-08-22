cbindgen --config cbindgen.toml -o include/zcash.h

cp ~/Projects/librustzcash/target/release/libzcash.dylib  ~/Projects/librustzcash/libzcash/include
