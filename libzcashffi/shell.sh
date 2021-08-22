cbindgen --config cbindgen.toml -o include/zcashffi.h

cargo build --release --all
cp ~/Projects/librustzcash/target/release/libzcashffi.dylib  ~/Projects/librustzcash/libzcashffi/include
