import bytecode from "./bytecode.js"

export const activate = () => init(bytecode)

let wasm;

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
/**
* @param {number} bits
* @param {number} min_size
* @param {number} max_size
* @param {number} window_size
* @returns {Rabin}
*/
export function create(bits, min_size, max_size, window_size) {
    var ret = wasm.create(bits, min_size, max_size, window_size);
    return Rabin.__wrap(ret);
}

const u32CvtShim = new Uint32Array(2);

const uint64CvtShim = new BigUint64Array(u32CvtShim.buffer);
/**
* @param {BigInt} mod_polynom
* @param {number} bits
* @param {number} min_size
* @param {number} max_size
* @param {number} window_size
* @returns {Rabin}
*/
export function createWithPolynomial(mod_polynom, bits, min_size, max_size, window_size) {
    uint64CvtShim[0] = mod_polynom;
    const low0 = u32CvtShim[0];
    const high0 = u32CvtShim[1];
    var ret = wasm.createWithPolynomial(low0, high0, bits, min_size, max_size, window_size);
    return Rabin.__wrap(ret);
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

let WASM_VECTOR_LEN = 0;

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1);
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function getArrayI32FromWasm0(ptr, len) {
    return getInt32Memory0().subarray(ptr / 4, ptr / 4 + len);
}
/**
* @param {Rabin} rabin
* @param {Uint8Array} bytes
* @param {boolean} end
* @returns {Int32Array}
*/
export function cut(rabin, bytes, end) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        _assertClass(rabin, Rabin);
        var ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.cut(retptr, rabin.ptr, ptr0, len0, end);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var v1 = getArrayI32FromWasm0(r0, r1).slice();
        wasm.__wbindgen_free(r0, r1 * 4);
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
*/
export class Rabin {

    static __wrap(ptr) {
        const obj = Object.create(Rabin.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_rabin_free(ptr);
    }
    /**
    */
    get min_size() {
        var ret = wasm.__wbg_get_rabin_min_size(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set min_size(arg0) {
        wasm.__wbg_set_rabin_min_size(this.ptr, arg0);
    }
    /**
    */
    get max_size() {
        var ret = wasm.__wbg_get_rabin_max_size(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set max_size(arg0) {
        wasm.__wbg_set_rabin_max_size(this.ptr, arg0);
    }
    /**
    */
    get window_size() {
        var ret = wasm.__wbg_get_rabin_window_size(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set window_size(arg0) {
        wasm.__wbg_set_rabin_window_size(this.ptr, arg0);
    }
    /**
    */
    get polynom_shift() {
        var ret = wasm.__wbg_get_rabin_polynom_shift(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set polynom_shift(arg0) {
        wasm.__wbg_set_rabin_polynom_shift(this.ptr, arg0);
    }
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
        input = new URL('wasm_bg.wasm', import.meta.url);
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    const { instance, module } = await load(input, imports)

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;

