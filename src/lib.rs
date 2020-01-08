extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

fn fact_recursive(num: i32, acc: i64) -> i64 {
    if num <= 0 {
        return 0;
    }
    if num == 1 {
        return acc;
    }
    return fact_recursive(num - 1, acc * num as i64);
}

#[wasm_bindgen]
pub fn fact(num: i32) -> i64 {
    return fact_recursive(num, 1);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn log_with_rust(s: String) {
    log(&s);
}