// The entry file of your WebAssembly module.


// import {logInteger} from "./env";

export function add(a: i32, b: i32): ArrayBuffer {
    const arr: ArrayBuffer = new ArrayBuffer(305)
    const view = new DataView(arr)
    view.setInt8(0, a as u8)
    view.setInt8(1, b as u8)
    view.setInt8(2, 15)
    return arr
}


export function draw(): ArrayBuffer {
    const arr: ArrayBuffer = new ArrayBuffer(18)
    const view = new DataView(arr)
    for (let i = 0; i < 18; i++) {
        view.setUint8(i, 255)
    }
    return arr
}
