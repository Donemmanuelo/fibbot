use num_bigint::BigInt;
use num_bigint::ToBigInt;

pub fn fibonacci(c: u128) -> BigInt {
let mut  a: BigInt = 0.to_bigint().expect("invalid input");
let mut  b: BigInt = 1.to_bigint().expect("invalid input");
 for _ in 0..c {
  let temp = a + b.clone() ;
  a = b;
  b = temp;
 }
  a
}

