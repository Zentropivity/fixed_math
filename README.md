This library implements analytic/trigonometric functions for [fixed point numbers](https://gitlab.com/tspiteri/fixed).

Implemented functions:

- `sqrt` from trait `FixedSqrt`
- `sin_cos`, `sin`, `cos`, `tan` from trait `FixedSinCos`
  - `FixedSinCos` is not implemented for fixed numbers with less than 7 integer bits (though this may change)
  - All calculations are made in degrees
    - except that there is a `sin_cos_rad` function which is very imprecise; check [source code](src/trig.rs) for why, feel free to fix it (its not a priority for me)

## Examples

There are traits and standalone functions, see examples on how to use them.

### Errors

Check the examples to see about how much error this produces.

Usually `sqrt` has an error of around 1-2 Delta.  
(Delta = the distance to the next representable number)

`sin_cos` may produce bigger errors, around 1-2 decimal places.

## Benchmarks

You can check or run the benchmarks in `benches`.

Here are some conclusions I've got to:

### SinCos

_2022-09-28_

Calculation time for sin_cos varies with the fixed number's byte size.

- I10F6: ~ 8ns
- I16F16: ~ 9ns
- I32F32: ~ 18ns
- I32F96: ~ 210ns

Notes:

- there are many different int bit / frac bit combinations; I did not test them  
  (int bits must be >= 10 (but maybe I can do something to relax that further))
- these are all calculations in degrees
- code was compiled with native cpu features
- go for FixedI32 instead of FixedI16 unless you are limited by memory much
- I did a benchmark in the same style on `cordic`'s `sin_cos` on FixedI64
  - keep in mind that `cordic` works with radians, I used the same angle values
  - so they can take `sin_cos` of a lot bigger angle on the same number representation
  - this crate was about 1.5-2 times faster on same angle sizes

## License

All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.

We use code modified from [cordic](https://github.com/sebcrozet/cordic), licensed as BSD-3-Clause:

- [LICENSE_CORDIC](third_party/LICENSE_CORDIC).

## Contributing

Unless you explicitly state otherwise,
any contribution intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, shall be dual licensed as above,
without any additional terms or conditions.
