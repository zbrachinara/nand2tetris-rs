import * as wasm from "hardware_simulator";

let width = document.documentElement.scrollWidth;
let height = document.documentElement.scrollHeight;

console.log(wasm.start(width, height));
