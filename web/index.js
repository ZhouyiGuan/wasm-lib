import { 
  hello_1, 
  hello_2,
  hello_3,
} from '../web_pkg';
  
console.log(hello_1("wasm"));
hello_2("wasm");
hello_3("hello from js:","wasm");