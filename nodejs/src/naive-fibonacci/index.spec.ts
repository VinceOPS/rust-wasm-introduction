import { naiveFibonacci } from '.';

describe('naiveFibonacci', () => {
  it('returns 0 for input 0', () => {
    expect(naiveFibonacci(0)).toEqual(0);
  });

  it('returns 1 for input 1', () => {
    expect(naiveFibonacci(1)).toEqual(1);
  });

  it('returns 55 for input 10', () => {
    expect(naiveFibonacci(10)).toEqual(55);
  });

  it('returns 832040 for input 30', () => {
    expect(naiveFibonacci(30)).toEqual(832040);
  });
});
