# Fixed Math

This library implements analytic/trigonometric functions for [fixed point numbers](https://gitlab.com/tspiteri/fixed).

Implemented functions:

- `sqrt` from trait `FixedSqrt`
  - `FixedSqrt` is not implemented for fixed numbers with less than 2 integer bits, but there are functions that work on those types: `sqrt_i1`, ...
- `sin_cos`, `sin`, `cos`, `tan` from trait `FixedSinCos`
  - `FixedSinCos` is not implemented for fixed numbers with less than 7 integer bits, but there are functions that work on some of those types: `sin_cos_i7`, ...
  - All calculations are made in degrees
    - except that there is a `sin_cos_rad` function which is very imprecise; check [source code](src/trig.rs) for why, feel free to fix it (its not a priority for me)

## Examples

There are traits and standalone functions, see examples on how to use them.

### Errors

Check the examples to see about how much error this implementation produces.

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
  - so they can take `sin_cos` of a lot bigger angle on the same number representation size
  - this crate was about 1.5-2 times faster on same angle sizes

_2023-07-30_

<details>
<summary>
System info:
</summary>

```
                   -`                    
                  .o+`                   --------
                 `ooo/                   OS: Arch Linux x86_64
                `+oooo:                  Host: X570 AORUS ELITE -CF
               `+oooooo:                 Kernel: 6.4.7-arch1-1
               -+oooooo+:                
             `/:-:++oooo+:               
            `/++++/+++++++:              Shell: fish 3.6.1
           `/++++++++++++++:             Resolution: 3840x2160
          `/+++ooooooooooooo/`           DE: Hyprland
         ./ooosssso++osssssso+`          
        .oossssso-````/ossssss+`         
       -osssssso.      :ssssssso.        Terminal: WezTerm
      :osssssss/        osssso+++.       CPU: AMD Ryzen 7 5800X (16) @ 3.800GHz
     /ossssssss/        +ssssooo/-       GPU: AMD ATI Radeon RX 7900 XT/7900 XTX
   `/ossssso+/:-        -:/+osssso+-     Memory: 6657MiB / 32014MiB
  `+sso+:-`                 `.-/+oso:
 `++:.                           `-/+/
 .`                                 `/
```
</details>

Performance is different now and **without native cpu features**:

- I10F6: ~ 430ps
- I16F16: ~ 15ns
- I32F32: ~ 25ns
- I32F96: ~ 255ns

**With native cpu features** : `RUSTFLAGS="-C target-cpu=native" cargo bench`:

- I10F6: ~ 420ps
- I16F16: ~ 8ns
- I32F32: ~ 22ns
- I32F96: ~ 222ns

Notes:

- performance regressed at I32F32 and I32F96
- performance improved a lot at I10F6, so we might consider using 16bit fixed point values where we do not need much precision
- _cordic benchmark not checked..._

## License

All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.

## Contributing

Unless you explicitly state otherwise,
any contribution intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, shall be dual licensed as above,
without any additional terms or conditions.

## Attribution

We use code modified from [cordic](https://github.com/sebcrozet/cordic), licensed as BSD-3-Clause:

- [LICENSE_CORDIC](third_party/LICENSE_CORDIC).
