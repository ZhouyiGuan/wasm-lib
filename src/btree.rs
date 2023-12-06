use super::*;
use std::collections::BTreeMap;

#[wasm_bindgen]
pub fn modify_buffer(buffer: &mut [u8]) {
    for i in 0..buffer.len() {
        buffer[i] = i as u8;
    }
}

// #[wasm_bindgen]
// pub struct BTree {
//     pub tree: BTreeMap<u32, u32>,
// }

// #[wasm_bindgen]
// impl BTree {
//     #[wasm_bindgen(constructor)]
//     pub fn new() -> BTree {
//         let mut tree = BTreeMap::new();
//         tree.insert(1, 2);
//         tree.insert(2, 3);
//         tree
//     }
// }

