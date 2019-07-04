import { performance, PerformanceObserver } from 'perf_hooks';
import * as MyWasmModule from 'rust-wasm-introduction';

import { naiveFibonacci } from '.';

/**
 * Very simplistic (stupid) benchmark to see how
 * the Wasm (from Rust) code performs against the JavaScript code.
 *
 * @param n nth term of the Fibonacci suite.
 */
function benchMe(n: number) {
  const obs = new PerformanceObserver(list => {
    const entry = list.getEntries()[0];
    console.log(`Fibonacci(${n}) - ${entry.name}: ${entry.duration}ms`);
  });
  obs.observe({ entryTypes: ['measure'] });

  performance.mark('Rust-a');
  MyWasmModule.naive_fibonacci(n);
  performance.mark('Rust-b');
  performance.measure('Rust', 'Rust-a', 'Rust-b');

  performance.mark('JS-a');
  naiveFibonacci(n);

  performance.mark('JS-b');
  performance.measure('JavaScript', 'JS-a', 'JS-b');

  obs.disconnect();
}

benchMe(20);
benchMe(43);
