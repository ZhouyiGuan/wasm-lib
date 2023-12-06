# node.js
(./)
wasm-pack build --target nodejs --out-dir node_pkg && node node/app.js

# web
(/web/)
npm install webpack webpack-cli --save-dev    
npm run serve