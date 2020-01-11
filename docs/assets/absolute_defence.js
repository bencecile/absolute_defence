
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

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    if (typeof(heap_next) !== 'number') throw new Error('corrupt heap');

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

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
/**
*/
export function first_time_setup() {
    wasm.first_time_setup();
}

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
/**
* @param {any} context
*/
export function init_context(context) {
    try {
        wasm.init_context(addBorrowedObject(context));
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error('expected a number argument');
}

function logError(e) {
    let error = (function () {
        try {
            return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
        } catch(_) {
            return "<failed to stringify thrown value>";
        }
    }());
    console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
    throw e;
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (typeof(arg) !== 'string') throw new Error('expected a string argument');

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);
        if (ret.read !== arg.length) throw new Error('failed to pass whole string');
        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}
/**
*/
export class GameState {

    constructor() {
        throw new Error('cannot invoke `new` directly');
    }

    static __wrap(ptr) {
        const obj = Object.create(GameState.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_gamestate_free(ptr);
    }
    /**
    * @returns {GameState}
    */
    static starting_state() {
        var ret = wasm.gamestate_starting_state();
        return GameState.__wrap(ret);
    }
    /**
    * @param {number} time_delta
    */
    update(time_delta) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        wasm.gamestate_update(this.ptr, time_delta);
    }
    /**
    * @param {any} context
    * @param {number} width
    * @param {number} height
    */
    render(context, width, height) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            _assertNum(this.ptr);
            wasm.gamestate_render(this.ptr, addBorrowedObject(context), width, height);
        } finally {
            heap[stack_pointer++] = undefined;
        }
    }
}

function init(module) {
    if (typeof module === 'undefined') {
        module = import.meta.url.replace(/\.js$/, '_bg.wasm');
    }
    let result;
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        var ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
        try {
            try {
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_free(arg0, arg1);
            }
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_new_59cb74e423758ede = function() {
        try {
            var ret = new Error();
            return addHeapObject(ret);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbg_stack_558ba5917b466edd = function(arg0, arg1) {
        try {
            var ret = getObject(arg1).stack;
            var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_begin_path_CanvasRenderingContext2D = function(arg0) {
        try {
            getObject(arg0).beginPath();
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_stroke_CanvasRenderingContext2D = function(arg0) {
        try {
            getObject(arg0).stroke();
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_set_stroke_style_CanvasRenderingContext2D = function(arg0, arg1) {
        try {
            getObject(arg0).strokeStyle = getObject(arg1);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_set_fill_style_CanvasRenderingContext2D = function(arg0, arg1) {
        try {
            getObject(arg0).fillStyle = getObject(arg1);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_set_line_width_CanvasRenderingContext2D = function(arg0, arg1) {
        try {
            getObject(arg0).lineWidth = arg1;
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_close_path_CanvasRenderingContext2D = function(arg0) {
        try {
            getObject(arg0).closePath();
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_line_to_CanvasRenderingContext2D = function(arg0, arg1, arg2) {
        try {
            getObject(arg0).lineTo(arg1, arg2);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_move_to_CanvasRenderingContext2D = function(arg0, arg1, arg2) {
        try {
            getObject(arg0).moveTo(arg1, arg2);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_clear_rect_CanvasRenderingContext2D = function(arg0, arg1, arg2, arg3, arg4) {
        try {
            getObject(arg0).clearRect(arg1, arg2, arg3, arg4);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_fill_rect_CanvasRenderingContext2D = function(arg0, arg1, arg2, arg3, arg4) {
        try {
            getObject(arg0).fillRect(arg1, arg2, arg3, arg4);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_stroke_rect_CanvasRenderingContext2D = function(arg0, arg1, arg2, arg3, arg4) {
        try {
            getObject(arg0).strokeRect(arg1, arg2, arg3, arg4);
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_restore_CanvasRenderingContext2D = function(arg0) {
        try {
            getObject(arg0).restore();
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_save_CanvasRenderingContext2D = function(arg0) {
        try {
            getObject(arg0).save();
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__widl_f_log_1_ = function(arg0) {
        try {
            console.log(getObject(arg0));
        } catch (e) {
            logError(e)
        }
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    if ((typeof URL === 'function' && module instanceof URL) || typeof module === 'string' || (typeof Request === 'function' && module instanceof Request)) {

        const response = fetch(module);
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            result = WebAssembly.instantiateStreaming(response, imports)
            .catch(e => {
                return response
                .then(r => {
                    if (r.headers.get('Content-Type') != 'application/wasm') {
                        console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                        return r.arrayBuffer();
                    } else {
                        throw e;
                    }
                })
                .then(bytes => WebAssembly.instantiate(bytes, imports));
            });
        } else {
            result = response
            .then(r => r.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes, imports));
        }
    } else {

        result = WebAssembly.instantiate(module, imports)
        .then(result => {
            if (result instanceof WebAssembly.Instance) {
                return { instance: result, module };
            } else {
                return result;
            }
        });
    }
    return result.then(({instance, module}) => {
        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;

        return wasm;
    });
}

export default init;

