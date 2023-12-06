import { 
  hello_1, 
  hello_2,
  hello_3,
} from '../node_pkg/wasm_lib.js';


console.log(hello_1("wasm"));
hello_2("wasm");
hello_3("hello from js:","wasm");


// 问一下 import/export ES6 
// import { 
//   test_import,
// } from '../node_pkg/wasm_lib.js';
// test_import();



