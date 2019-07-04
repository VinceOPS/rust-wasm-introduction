import * as MyWasmModule from 'rust-wasm-introduction';

const account = MyWasmModule.Account.new(0);
account.deposit(500);
document.body.append(`Current cash: ${account.get_balance()}`);
