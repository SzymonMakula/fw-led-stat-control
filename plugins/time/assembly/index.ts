// The entry file of your WebAssembly module.


// import {logInteger} from "./env";

import {get_epoch_time} from "./env";

const HEIGHT: i32 = 4;
const WIDTH: i32 = 8;
const PICTURE_LEN: i32 = HEIGHT * WIDTH;


function createIndex(row: i32, col: i32): i32 {
    return row * WIDTH + col
}

const LIT_VAL = 255;
const DIM_VAL = 10;


function formatHours(date: Date): Array<i32> {
    const arr = new Array<i32>(PICTURE_LEN);


    // Dim all lights before setting individual lights
    arr[createIndex(2, 0)] = DIM_VAL
    arr[createIndex(3, 0)] = DIM_VAL

    arr[createIndex(0, 1)] = DIM_VAL
    arr[createIndex(1, 1)] = DIM_VAL
    arr[createIndex(2, 1)] = DIM_VAL
    arr[createIndex(3, 1)] = DIM_VAL

    // Lit 10/20 hour
    const hour = date.getUTCHours();
    const secondDecimalDigit: i32 = floor(hour / 10);

    if (secondDecimalDigit == 2) {
        arr[createIndex(2, 0)] = 255
    } else if (secondDecimalDigit == 1) {
        arr[createIndex(3, 0)] = 255
    }

    const FIRST_DECIMAL_INDICES: Array<i32> = [createIndex(0, 1), createIndex(1, 1), createIndex(2, 1), createIndex(3, 1)]

    const firstDecimalDigit: Array<number> = (hour % 10).toString(2).padStart(4, "0").split("").map<f64>(val => Number.parseInt(val, 2));

    for (let i = 0; i < firstDecimalDigit.length; i++) {
        arr[FIRST_DECIMAL_INDICES[i]] = firstDecimalDigit[i] > 0 ? LIT_VAL : DIM_VAL
    }

    return arr
}

function formatMinutes(date: Date): Array<i32> {
    const arr = new Array<i32>(PICTURE_LEN);
    const minutes = date.getUTCMinutes();


    // Dim all lights before setting individual lights
    arr[createIndex(1, 3)] = DIM_VAL
    arr[createIndex(2, 3)] = DIM_VAL
    arr[createIndex(3, 3)] = DIM_VAL

    arr[createIndex(0, 4)] = DIM_VAL
    arr[createIndex(1, 4)] = DIM_VAL
    arr[createIndex(2, 4)] = DIM_VAL
    arr[createIndex(3, 4)] = DIM_VAL

    const FIRST_DECIMAL_INDICES: Array<i32> = [createIndex(1, 3), createIndex(2, 3), createIndex(3, 3)]
    const firstDecimalDigit: Array<number> = floor(minutes / 10).toString(2).padStart(3, "0").split("").map<f64>(val => Number.parseInt(val, 2));

    for (let i = 0; i < firstDecimalDigit.length; i++) {
        arr[FIRST_DECIMAL_INDICES[i]] = firstDecimalDigit[i] > 0 ? LIT_VAL : DIM_VAL
    }

    const SECOND_DECIMAL_INDICES: Array<i32> = [createIndex(0, 4), createIndex(1, 4), createIndex(2, 4), createIndex(3, 4)]
    const secondDecimalDigit: Array<number> = floor(minutes % 10).toString(2).padStart(4, "0").split("").map<f64>(val => Number.parseInt(val, 2));

    for (let i = 0; i < secondDecimalDigit.length; i++) {
        arr[SECOND_DECIMAL_INDICES[i]] = secondDecimalDigit[i] > 0 ? LIT_VAL : DIM_VAL
    }

    return arr
}

function formatSeconds(date: Date): Array<i32> {
    const arr = new Array<i32>(PICTURE_LEN);


    const FIRST_DECIMAL_INDICES: Array<i32> = [createIndex(1, 6), createIndex(2, 6), createIndex(3, 6)]
    const SECOND_DECIMAL_INDICES: Array<i32> = [createIndex(0, 7), createIndex(1, 7), createIndex(2, 7), createIndex(3, 7)]


    // DIM all lights

    for (let i = 0; i < FIRST_DECIMAL_INDICES.length; i++) {
        arr[FIRST_DECIMAL_INDICES[i]] = DIM_VAL
    }


    for (let i = 0; i < SECOND_DECIMAL_INDICES.length; i++) {
        arr[SECOND_DECIMAL_INDICES[i]] = DIM_VAL
    }


    const seconds = date.getUTCSeconds()

    const firstDecimalDigit: Array<number> = floor(seconds / 10).toString(2).padStart(3, "0").split("").map<f64>(val => Number.parseInt(val, 2));
    for (let i = 0; i < firstDecimalDigit.length; i++) {
        arr[FIRST_DECIMAL_INDICES[i]] = firstDecimalDigit[i] > 0 ? LIT_VAL : DIM_VAL
    }

    const secondDecimalDigit: Array<number> = (seconds % 10).toString(2).padStart(4, "0").split("").map<f64>(val => Number.parseInt(val, 2));

    for (let i = 0; i < secondDecimalDigit.length; i++) {
        arr[SECOND_DECIMAL_INDICES[i]] = secondDecimalDigit[i] > 0 ? LIT_VAL : DIM_VAL
    }

    return arr


}

export function draw(): ArrayBuffer {
    const epochTime = get_epoch_time()

    const date = new Date(epochTime * 1000)
    const arr: ArrayBuffer = new ArrayBuffer(PICTURE_LEN)
    const view = new DataView(arr)

    const hoursArray = formatHours(date)
    for (let i = 0; i < PICTURE_LEN; i++) {
        if (hoursArray[i] > 0) view.setUint8(i, hoursArray[i] as u8)
    }

    const minutesArray = formatMinutes(date)
    for (let i = 0; i < PICTURE_LEN; i++) {
        if (minutesArray[i] > 0) view.setUint8(i, minutesArray[i] as u8)
    }

    const secondsArray = formatSeconds(date)
    for (let i = 0; i < PICTURE_LEN; i++) {
        if (secondsArray[i] > 0) view.setUint8(i, secondsArray[i] as u8)
    }

    return arr
}
