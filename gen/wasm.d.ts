/* tslint:disable */
/* eslint-disable */
/**
* @param {number} bits
* @param {number} min_size
* @param {number} max_size
* @param {number} window_size
* @returns {Rabin}
*/
export function create(bits: number, min_size: number, max_size: number, window_size: number): Rabin;
/**
* @param {BigInt} mod_polynom
* @param {number} bits
* @param {number} min_size
* @param {number} max_size
* @param {number} window_size
* @returns {Rabin}
*/
export function createWithPolynomial(mod_polynom: BigInt, bits: number, min_size: number, max_size: number, window_size: number): Rabin;
/**
* @param {Rabin} rabin
* @param {Uint8Array} bytes
* @param {boolean} end
* @returns {Int32Array}
*/
export function cut(rabin: Rabin, bytes: Uint8Array, end: boolean): Int32Array;
/**
*/
export class Rabin {
  free(): void;
/**
*/
  max_size: number;
/**
*/
  min_size: number;
/**
*/
  polynom_shift: number;
/**
*/
  window_size: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly create: (a: number, b: number, c: number, d: number) => number;
  readonly createWithPolynomial: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly cut: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbg_rabin_free: (a: number) => void;
  readonly __wbg_get_rabin_min_size: (a: number) => number;
  readonly __wbg_set_rabin_min_size: (a: number, b: number) => void;
  readonly __wbg_get_rabin_max_size: (a: number) => number;
  readonly __wbg_set_rabin_max_size: (a: number, b: number) => void;
  readonly __wbg_get_rabin_window_size: (a: number) => number;
  readonly __wbg_set_rabin_window_size: (a: number, b: number) => void;
  readonly __wbg_get_rabin_polynom_shift: (a: number) => number;
  readonly __wbg_set_rabin_polynom_shift: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
declare function activate(): ReturnType<typeof init>
