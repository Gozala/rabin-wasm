import * as Rabin from "./gen/wasm.js"

let wait = Rabin.activate().then(() => {
  wait = {
    then: fn => fn(),
  }
})

/**
 * @param {number} bits
 * @param {number} minSize
 * @param {number} maxSize
 * @param {number} windowSize
 */
export const create = (bits, minSize, maxSize, windowSize) =>
  wait.then(() => Rabin.create(bits, minSize, maxSize, windowSize))

/**
 * @param {number} bits
 * @param {number} minSize
 * @param {number} maxSize
 * @param {number} windowSize
 */
export const createWithPolynom = (
  polynom,
  bits,
  minSize,
  maxSize,
  windowSize
) =>
  wait.then(() =>
    Rabin.createWithPolynomial(polynom, bits, minSize, maxSize, windowSize)
  )

export const cut = Rabin.cut
const Type = Rabin.Rabin
export { Type as Rabin }
