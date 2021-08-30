export function wasm_bindgen_initialize( memory, table, alloc, free ) {
        var Module = {};
        Module.web_malloc = alloc;
        Module.web_free = free;
        Module.web_table = table;
        Object.defineProperty( Module, "HEAP8", {
            get: function() { return new Int8Array( memory.buffer ); }
        });
        Object.defineProperty( Module, "HEAP16", {
            get: function() { return new Int16Array( memory.buffer ); }
        });
        Object.defineProperty( Module, "HEAP32", {
            get: function() { return new Int32Array( memory.buffer ); }
        });
        Object.defineProperty( Module, "HEAPU8", {
            get: function() { return new Uint8Array( memory.buffer ); }
        });
        Object.defineProperty( Module, "HEAPU16", {
            get: function() { return new Uint16Array( memory.buffer ); }
        });
        Object.defineProperty( Module, "HEAPU32", {
            get: function() { return new Uint32Array( memory.buffer ); }
        });
        Object.defineProperty( Module, "HEAPF32", {
            get: function() { return new Float32Array( memory.buffer ); }
        });
        Object.defineProperty( Module, "HEAPF64", {
            get: function() { return new Float64Array( memory.buffer ); }
        });
        return Module;
    }