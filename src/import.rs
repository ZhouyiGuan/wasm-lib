// use super::*;


// #[wasm_bindgen(module = "/node/export.js")]
// extern "C" {
//     fn name() -> String;

//     type MyClass;
//     #[wasm_bindgen(constructor)]
//     fn new() -> MyClass;
//     #[wasm_bindgen(method, getter)]
//     fn number(this: &MyClass) -> u32;
//     #[wasm_bindgen(method, setter)]
//     fn set_number(this: &MyClass, number: u32) -> MyClass;
//     #[wasm_bindgen(method)]
//     fn render(this: &MyClass) -> String;
// }

// #[wasm_bindgen]
// pub fn test_import() {
//     console_log!("hello from {}!", name()); 

//     let x = MyClass::new();
//     assert_eq!(x.number(), 42);
//     x.set_number(10);
//     console_log!("{}",x.render());
// }
