export function naiveFibonacci(n: number): number {
  if (n <= 1) {
    return n;
  }

  return naiveFibonacci(n - 1) + naiveFibonacci(n - 2);
}
