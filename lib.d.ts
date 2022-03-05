import { Rabin } from "./gen/wasm.js"

export { Rabin }

export function create(
  bits: number,
  minSize: number,
  maxSize: number,
  windowSize: number
): Rabin | Promise<Rabin>

export function createWithPolynom(
  polynom: BigInt,
  bits: number,
  minSize: number,
  maxSize: number,
  windowSize: number
): Rabin | Promise<Rabin>

export function cut(rabin: Rabin, bytes: Uint8Array, end: boolean): Int32Array
