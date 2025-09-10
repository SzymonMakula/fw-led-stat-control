import {get_battery_state_of_charge} from "./env";

export function add(a: i32, b: i32): ArrayBuffer {
    const arr: ArrayBuffer = new ArrayBuffer(305)
    const view = new DataView(arr)
    view.setInt8(0, a as u8)
    view.setInt8(1, b as u8)
    view.setInt8(2, 15)
    return arr
}

function isRowLit(row: i32, state_of_charge: f32): boolean {
    const percentile = floor(state_of_charge * 10) as i32
    return percentile > row
}

const HEIGHT: i32 = 10;
const WIDTH: i32 = 5;
const PICTURE_LEN: i32 = HEIGHT * WIDTH;

export function draw(): ArrayBuffer {

    const state_of_charge = get_battery_state_of_charge()
    const arr: ArrayBuffer = new ArrayBuffer(PICTURE_LEN)
    const view = new DataView(arr)
    for (let i = PICTURE_LEN - 1; i >= 0; i--) {
        let current_row = floor(i / WIDTH)
        const isLit = isRowLit(current_row, state_of_charge)
        view.setUint8(PICTURE_LEN - 1 - i, isLit ? 255 : 10)
    }
    return arr
}
