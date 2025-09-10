import {get_global_cpu_usage} from "./env";


function isRowLit(row: i32, cpu_usage: f32): boolean {
    const percentile = floor(cpu_usage / 10) as i32
    return percentile > row
}

const PICTURE_LEN: i32 = 20;
const WIDTH: i32 = 2;

export function draw(): ArrayBuffer {

    const cpu_usage = get_global_cpu_usage()
    const arr: ArrayBuffer = new ArrayBuffer(PICTURE_LEN)
    const view = new DataView(arr)
    for (let i = PICTURE_LEN - 1; i >= 0; i--) {
        // const val: u8 = Math.floor(Math.random() * 255)
        let current_row = floor(i / WIDTH)
        const isLit = isRowLit(current_row, cpu_usage)
        view.setUint8(PICTURE_LEN - 1 - i, isLit ? 255 : 10)
    }
    return arr
}
