# mtots webassembly bindings

Actually, the logic here should be enough for external C bindings as well
if desired

However, the main goal here is to port mtots to non-desktop platforms
(i.e. web and mobile) by targeting webassembly

The relevant build comamnd is:

    cargo build --release --target=wasm32-unknown-unknown

and you can check out what it generated with:

    wasm-objdump -x target/wasm32-unknown-unknown/release/mtots_wa.wasm
