/* tslint:disable */
/* eslint-disable */
/**
*/
export function init_panic_hook(): void;
/**
* Entry point invoked by `worker.js`, a bit of a hack but see the "TODO" above
* about `worker.js` in general.
* @param {number} ptr 
*/
export function child_entry_point(ptr: number): void;
/**
*/
export class TowerDefenceBuilder {
  free(): void;
/**
* @returns {TowerDefenceBuilder} 
*/
  static new(): TowerDefenceBuilder;
/**
* @param {HTMLCanvasElement} canvas 
* @returns {TowerDefenceBuilder} 
*/
  with_canvas(canvas: HTMLCanvasElement): TowerDefenceBuilder;
/**
* @param {string} input_bindings_str 
* @returns {TowerDefenceBuilder} 
*/
  with_input_bindings(input_bindings_str: string): TowerDefenceBuilder;
/**
*/
  run(): void;
}
/**
* The `WorkerPool`. This is a special type of thread pool that works on wasm and provide a way to
* run work they way rayon does it.
*/
export class WorkerPool {
  free(): void;
/**
* Creates a new `WorkerPool` which immediately creates `initial` workers.
*
* The pool created here can be used over a long period of time, and it
* will be initially primed with `initial` workers. Currently workers are
* never released or gc'd until the whole pool is destroyed.
*
* # Errors
*
* Returns any error that may happen while a JS web worker is created and a
* message is sent to it.
* @param {number} initial 
*/
  constructor(initial: number);
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly __wbg_towerdefencebuilder_free: (a: number) => void;
  readonly towerdefencebuilder_new: () => number;
  readonly towerdefencebuilder_with_canvas: (a: number, b: number) => number;
  readonly towerdefencebuilder_with_input_bindings: (a: number, b: number, c: number) => number;
  readonly towerdefencebuilder_run: (a: number) => void;
  readonly init_panic_hook: () => void;
  readonly __wbg_workerpool_free: (a: number) => void;
  readonly workerpool_new: (a: number) => number;
  readonly child_entry_point: (a: number) => void;
  readonly atou8_range: (a: number, b: number) => number;
  readonly atou16_range: (a: number, b: number) => number;
  readonly atou32_range: (a: number, b: number) => number;
  readonly atou64_range: (a: number, b: number) => number;
  readonly atousize_range: (a: number, b: number) => number;
  readonly atoi8_range: (a: number, b: number) => number;
  readonly atoi16_range: (a: number, b: number) => number;
  readonly atoi32_range: (a: number, b: number) => number;
  readonly atoi64_range: (a: number, b: number) => number;
  readonly atoisize_range: (a: number, b: number) => number;
  readonly try_atou8_range: (a: number, b: number, c: number) => void;
  readonly try_atou16_range: (a: number, b: number, c: number) => void;
  readonly try_atou32_range: (a: number, b: number, c: number) => void;
  readonly try_atou64_range: (a: number, b: number, c: number) => void;
  readonly try_atoi8_range: (a: number, b: number, c: number) => void;
  readonly try_atoi16_range: (a: number, b: number, c: number) => void;
  readonly try_atoi32_range: (a: number, b: number, c: number) => void;
  readonly try_atoi64_range: (a: number, b: number, c: number) => void;
  readonly atou128_range: (a: number, b: number, c: number) => void;
  readonly atoi128_range: (a: number, b: number, c: number) => void;
  readonly try_atou128_range: (a: number, b: number, c: number) => void;
  readonly try_atoi128_range: (a: number, b: number, c: number) => void;
  readonly try_atoisize_range: (a: number, b: number, c: number) => void;
  readonly try_atousize_range: (a: number, b: number, c: number) => void;
  readonly is_success: (a: number, b: number) => number;
  readonly is_overflow: (a: number, b: number) => number;
  readonly is_invalid_digit: (a: number, b: number) => number;
  readonly is_empty: (a: number, b: number) => number;
  readonly get_nan_string_ffi: (a: number, b: number) => number;
  readonly set_nan_string_ffi: (a: number, b: number) => number;
  readonly get_inf_string_ffi: (a: number, b: number) => number;
  readonly set_inf_string_ffi: (a: number, b: number) => number;
  readonly get_infinity_string_ffi: (a: number, b: number) => number;
  readonly set_infinity_string_ffi: (a: number, b: number) => number;
  readonly f32toa_range: (a: number, b: number, c: number) => number;
  readonly f64toa_range: (a: number, b: number, c: number) => number;
  readonly u8toa_range: (a: number, b: number, c: number) => number;
  readonly u16toa_range: (a: number, b: number, c: number) => number;
  readonly u32toa_range: (a: number, b: number, c: number) => number;
  readonly u64toa_range: (a: number, b: number, c: number) => number;
  readonly usizetoa_range: (a: number, b: number, c: number) => number;
  readonly i8toa_range: (a: number, b: number, c: number) => number;
  readonly i16toa_range: (a: number, b: number, c: number) => number;
  readonly i32toa_range: (a: number, b: number, c: number) => number;
  readonly i64toa_range: (a: number, b: number, c: number) => number;
  readonly isizetoa_range: (a: number, b: number, c: number) => number;
  readonly u128toa_range: (a: number, b: number, c: number, d: number) => number;
  readonly i128toa_range: (a: number, b: number, c: number, d: number) => number;
  readonly atof32_range: (a: number, b: number) => number;
  readonly atof64_range: (a: number, b: number) => number;
  readonly atof32_lossy_range: (a: number, b: number) => number;
  readonly atof64_lossy_range: (a: number, b: number) => number;
  readonly try_atof32_range: (a: number, b: number, c: number) => void;
  readonly try_atof64_range: (a: number, b: number, c: number) => void;
  readonly try_atof32_lossy_range: (a: number, b: number, c: number) => void;
  readonly try_atof64_lossy_range: (a: number, b: number, c: number) => void;
  readonly __wbindgen_export_0: WebAssembly.Memory;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_3: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h23029b35d22b85d7: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h5f7c82076eb26f92: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h6d1af2684d2c1aa8: (a: number, b: number) => void;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
* @param {WebAssembly.Memory} maybe_memory
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>, maybe_memory: WebAssembly.Memory): Promise<InitOutput>;
        