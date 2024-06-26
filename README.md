# rev_bits

A Rust library for reversing bits for a specified range of an integer (`u32` or `u64`).

## Example

```rust
let x: u32 = 0xF0FFA000;
let y = reverse(x, 8..16);
println!("original: {:08X}", x);
println!(" changed: {}", "....xx..");
println!("reversed: {:08X}", y);
```

Gives:

```txt
original: F0FFA000
 changed: ....xx..
reversed: F0FF0500
```

## Crate

`rev_bits` is [available on crates.io][1]:

```sh
cargo add rev_bits
```

[1]: https://crates.io/crates/rev_bits

## Test

`rev_bits` has extensive property-based tests thanks to [QuickCheck]:

```sh
cargo test
```

[QuickCheck]: https://github.com/BurntSushi/quickcheck

## Benchmark

`rev_bits` has benchmarks thanks to [Criterion.rs]:

```sh
cargo bench
```

[Criterion.rs]: https://github.com/bheisler/criterion.rs
