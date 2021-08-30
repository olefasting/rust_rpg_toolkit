import { wasm_bindgen_initialize } from './snippets/stdweb-bb142200b065bd55/inline119.js';
import { __cargo_web_snippet_5c3091ae7fa9c42123eec37f64de99a5808e7ef2 } from './snippets/stdweb-bb142200b065bd55/inline347.js';
import { __cargo_web_snippet_8c32019649bb581b1b742eeedfc410e2bedd56a6 } from './snippets/stdweb-bb142200b065bd55/inline474.js';
import { __cargo_web_snippet_5c57e16ebd22655f976d87fae8039e282d7bab59 } from './snippets/stdweb-bb142200b065bd55/inline542.js';
import { __cargo_web_snippet_ecd8f83530fd9b57edbdc4822b4ea5b373e3a927 } from './snippets/stdweb-bb142200b065bd55/inline550.js';
import { __cargo_web_snippet_72fc447820458c720c68d0d8e078ede631edd723 } from './snippets/stdweb-bb142200b065bd55/inline565.js';
import { __cargo_web_snippet_97495987af1720d8a9a923fa4683a7b683e3acd6 } from './snippets/stdweb-bb142200b065bd55/inline566.js';
import { __cargo_web_snippet_dc2fd915bd92f9e9c6a3bd15174f1414eee3dbaf } from './snippets/stdweb-bb142200b065bd55/inline567.js';
import { __cargo_web_snippet_1c30acb32a1994a07c75e804ae9855b43f191d63 } from './snippets/stdweb-bb142200b065bd55/inline568.js';
import { __cargo_web_snippet_80d6d56760c65e49b7be8b6b01c1ea861b046bf0 } from './snippets/stdweb-bb142200b065bd55/inline66.js';
import { __cargo_web_snippet_f91e3759ea8e40c15404c8b089c74fe76b61421e } from './snippets/stdweb-bb142200b065bd55/inline699.js';
init.set_wasm = w => wasm = w;

let wasm;

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function makeClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_0.get(state.dtor)(state.a, state.b);
                state.a = 0;

            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_12(arg0, arg1, arg2, arg3) {
    wasm._dyn_core__ops__function__Fn__A_B___Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h013568ac65a625fb(arg0, arg1, arg2, arg3);
}

function __wbg_adapter_15(arg0, arg1, arg2) {
    var ret = wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h19c42730ef8aea5a(arg0, arg1, arg2);
    return ret;
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('example_project_bg.wasm', import.meta.url);
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        var ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_cargowebsnippet80d6d56760c65e49b7be8b6b01c1ea861b046bf0_5a8953894b8affd6 = function(arg0, arg1) {
        __cargo_web_snippet_80d6d56760c65e49b7be8b6b01c1ea861b046bf0(takeObject(arg0), arg1);
    };
    imports.wbg.__wbg_wasmbindgeninitialize_c1c4df6b494511ad = function(arg0, arg1, arg2, arg3) {
        var ret = wasm_bindgen_initialize(takeObject(arg0), takeObject(arg1), getObject(arg2), getObject(arg3));
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_cargowebsnippet5c3091ae7fa9c42123eec37f64de99a5808e7ef2_fc0bd84666f3fba5 = function(arg0, arg1) {
        var ret = __cargo_web_snippet_5c3091ae7fa9c42123eec37f64de99a5808e7ef2(takeObject(arg0), arg1);
        return ret;
    };
    imports.wbg.__wbg_cargowebsnippet8c32019649bb581b1b742eeedfc410e2bedd56a6_fe72322db9f33c63 = function(arg0, arg1, arg2) {
        __cargo_web_snippet_8c32019649bb581b1b742eeedfc410e2bedd56a6(takeObject(arg0), arg1, arg2);
    };
    imports.wbg.__wbg_cargowebsnippetecd8f83530fd9b57edbdc4822b4ea5b373e3a927_4b05cce5d27da5ee = function(arg0, arg1) {
        var ret = __cargo_web_snippet_ecd8f83530fd9b57edbdc4822b4ea5b373e3a927(takeObject(arg0), arg1);
        return ret;
    };
    imports.wbg.__wbg_cargowebsnippet1c30acb32a1994a07c75e804ae9855b43f191d63_6d353463ef525961 = function(arg0) {
        __cargo_web_snippet_1c30acb32a1994a07c75e804ae9855b43f191d63(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippetdc2fd915bd92f9e9c6a3bd15174f1414eee3dbaf_ce5c721cab10d020 = function(arg0) {
        __cargo_web_snippet_dc2fd915bd92f9e9c6a3bd15174f1414eee3dbaf(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippet97495987af1720d8a9a923fa4683a7b683e3acd6_a438202dc16f44c0 = function(arg0, arg1, arg2) {
        __cargo_web_snippet_97495987af1720d8a9a923fa4683a7b683e3acd6(takeObject(arg0), arg1, arg2);
    };
    imports.wbg.__wbg_cargowebsnippet72fc447820458c720c68d0d8e078ede631edd723_ece3da0a4474dbeb = function(arg0, arg1, arg2, arg3) {
        __cargo_web_snippet_72fc447820458c720c68d0d8e078ede631edd723(takeObject(arg0), arg1, arg2, arg3);
    };
    imports.wbg.__wbg_cargowebsnippet5c57e16ebd22655f976d87fae8039e282d7bab59_ad29da5ae967b2d2 = function(arg0, arg1) {
        __cargo_web_snippet_5c57e16ebd22655f976d87fae8039e282d7bab59(takeObject(arg0), arg1);
    };
    imports.wbg.__wbg_cargowebsnippetf91e3759ea8e40c15404c8b089c74fe76b61421e_9d39745186b96213 = function(arg0, arg1) {
        __cargo_web_snippet_f91e3759ea8e40c15404c8b089c74fe76b61421e(takeObject(arg0), arg1);
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_memory = function() {
        var ret = wasm.memory;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_function_table = function() {
        var ret = wasm.__wbindgen_export_0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper15494 = function(arg0, arg1, arg2) {
        var ret = makeClosure(arg0, arg1, 633, __wbg_adapter_12);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper15496 = function(arg0, arg1, arg2) {
        var ret = makeClosure(arg0, arg1, 633, __wbg_adapter_15);
        return addHeapObject(ret);
    };
    return imports;

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }



    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    wasm.__wbindgen_start();
    return wasm;
}

export default init;

