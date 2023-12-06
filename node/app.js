import { 
  hello_1, 
  hello_2,
  hello_3,
} from '../node_pkg/wasm_lib.js';

console.log(hello_1("wasm"));
hello_2("wasm");
hello_3("hello from js:","wasm");


// const buffer = new ArrayBuffer(500 * 1024 * 1024); 
// const used = process.memoryUsage();
// for (let key in used) {
//   console.log(`${key} ${Math.round(used[key] / 1024 / 1024 * 100) / 100} MB`);
// };
// modify_buffer(buffer);
// for (let key in used) {
//     console.log(`${key} ${Math.round(used[key] / 1024 / 1024 * 100) / 100} MB`);
// };


// console.time("js");
// console.timeEnd("js");

