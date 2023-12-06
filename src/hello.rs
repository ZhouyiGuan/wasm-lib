use super::*;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_many(a: &str, b: &str);
}

#[macro_export]
macro_rules! console_log {
  ($($t:tt)*) => (hello::log(&format_args!($($t)*).to_string()))
}
#[macro_export]
macro_rules! console_log2 {
  ($a:expr, $b:expr) => {
      log_many(&$a.to_string(), &$b.to_string())
  };
}


#[wasm_bindgen]
pub fn hello_1(data: String) -> String {
  format!("hello from rust: {}", data)
}
#[wasm_bindgen]
pub fn hello_2(data: String) {
  console_log!("hello from js: {}",data);
}
#[wasm_bindgen]
pub fn hello_3(data1: String,data2: String) {
  console_log2!(data1,data2);
}

