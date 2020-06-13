// Try loading the webassembly module from node
// and interacting with it

const fs = require('fs');

const wa_path = (
    __dirname +
    '/../target/wasm32-unknown-unknown/release/mtots_wa.wasm'
);


const wa_buffer = fs.readFileSync(wa_path);
console.log(wa_buffer);

WebAssembly.instantiate(wa_buffer, {}).then(results => {
    const module = results.module;
    const instance = results.instance;
    console.log(['instance-exports', instance.exports])
    console.log(['imports', WebAssembly.Module.exports(module)])
    console.log(['exports', WebAssembly.Module.exports(module)])
    console.log(['results', results])

    const mtots = instance.exports.mtots_new();
    console.log(['mtots', mtots])
})

