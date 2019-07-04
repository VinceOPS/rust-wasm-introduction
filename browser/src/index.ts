// dynamic import because WebAssembly download and compilation must happen asynchronously
import('./load-wasm').catch(e => console.error('Error importing load-wasm:', e));
