use wasm_bindgen::prelude::*;
use cfg_if::cfg_if;


cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        use wee_alloc::WeeAlloc;
        #[global_allocator]
        static ALLOC: WeeAlloc = WeeAlloc::INIT;
    }
}


mod utils;

mod hello;

mod buffer;

mod import;