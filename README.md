# node.js
./
wasm-pack build --target nodejs --out-dir node_pkg && node node/app.js

# web
/web
npm install webpack webpack-cli --save-dev    
npm run serve(使用webpack调试服务器运行)
npm run build(打包项目)