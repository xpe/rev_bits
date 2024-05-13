use std::ops::RangeBounds;

use super::{to_start_end, ALL_ONES, BITS};

/// Returns the input where bits in the range are reversed.
///
/// For example:
///     `abcdefghiABCDEFjkmlmnopqrtuvwxyz`
/// becomes
///     `abcdefghiFEDCBAjklmnopqrstuvwxyz`
/// for the range specified as: `17..23` or `17..=22`.
pub fn reverse(input: u64, range: impl RangeBounds<u32>) -> u64 {
    let (start, end) = to_start_end(range);
    if end > start {
        reverse_closed_interval(input, start, end)
    } else {
        input
    }
}

/// Returns the input with bits in the closed interval [min, max] reversed.
///
/// For example:
///     `abcdefghiABCDEFjkmlmnopqrtuvwxyz`
/// becomes
///     `abcdefghiFEDCBAjklmnopqrstuvwxyz`
/// for min=16, max=23
fn reverse_closed_interval(input: u64, min: u32, max: u32) -> u64 {
    // The comments use the following input as an example:
    //     abcdefghiABCDEFjkmlmnopqrtuvwxyz
    //
    // - uppercase letters: bits to reverse
    // - lowercase letters: bits that won't be changed
    //
    // Bits are indexed by counting from the right, so index 0 is the least
    // significant bit:
    //     abcdefghiABCDEFjkmlmnopqrtuvwxyz
    //              ^    ^
    //              |    |
    //       22 = idx    min = 17
    assert!(min < max);

    //     abcdefghiABCDEFjkmlmnopqrtuvwxyz
    //              ------
    //            length = 6
    let length = max + 1 - min;

    // Create a ones mask for the [min, max] interval.
    //     11111111111111111111111111111111
    //     111111<-------------------------
    //                   BITS-length = 32-6=26
    //
    //
    //     -------->111111.................
    //         |
    //     BITS-(max+1) = 32-(22+1) = 9
    //
    //     00000000011111100000000000000000
    let ones_mask = (ALL_ONES << (BITS - length)) >> (BITS - (max + 1));

    // Calculate inputs bits to reverse:
    //     000000000ABCDEF00000000000000000
    let input_to_reverse = input & ones_mask;

    // Create a zeros mask for the [min, max] interval.
    //     11111111100000011111111111111111
    let zeros_mask = !ones_mask;

    // Calculate inputs bits to keep unchanged:
    //     abcdefghi000000jkmlmnopqrtuvwxyz
    let input_to_keep = input & zeros_mask;

    // Align `input_to_reverse` to the right, preparing for reversal.
    //     .................abcdefghiABCDEF
    //     00000000000000000000000000ABCDEF
    //     ---------------->
    //         min = 17     ---------------
    //                      input_to_reverse
    let to_reverse = input_to_reverse >> min;

    // Reverse part of the input. The useful portion is now on the left.
    //     FEDCBA00000000000000000000000000
    //     ------
    //     length
    let reversed_on_left = to_reverse.reverse_bits();

    // Put the reversed bits in the correct position.
    //     000000000FEDCBA00000000000000000
    //     -------->
    //         |
    //     32-(max+1) = 32-(22+1) = 9
    let reversed_in_position = reversed_on_left >> (BITS - (max + 1));

    // Return the final result
    //     abcdefghiFEDCBAjklmnopqrstuvwxyz
    input_to_keep | reversed_in_position
}

#[cfg(test)]
mod tests {
    use quickcheck::{QuickCheck, TestResult};

    use crate::u64::reverse_u64;

    use super::*;

    #[test]
    fn reverse_compared_to_reference_implementation() {
        fn versus_ref_impl(x: u64, a: u32, b: u32) -> TestResult {
            if a >= BITS || b >= BITS || a >= b {
                return TestResult::discard();
            }
            let y1 = reverse_closed_interval(x, a, b);
            let y2 = reverse_u64(x, a..=b);
            TestResult::from_bool(y1 == y2)
        }
        QuickCheck::new()
            .tests(25_000)
            .max_tests(1_000_000)
            .quickcheck(versus_ref_impl as fn(u64, u32, u32) -> TestResult);
    }

    #[test]
    fn reverse_closed_interval_outside_interval_unchanged() {
        fn outside_interval_unchanged(x: u64, a: u32, b: u32) -> TestResult {
            if a >= BITS || b >= BITS || a >= b {
                return TestResult::discard();
            }
            let reversed = reverse_closed_interval(x, a, b);
            let compared = x ^ reversed;
            let b1 = b + 1;
            let left_ones = if b1 == BITS { 0 } else { ALL_ONES << b1 };
            let right_ones = if a == 0 { 0 } else { ALL_ONES >> (BITS - a) };
            let ones_mask = left_ones | right_ones;
            let unchanged = compared & ones_mask == 0;
            TestResult::from_bool(unchanged)
        }
        QuickCheck::new()
            .tests(25_000)
            .max_tests(1_000_000)
            .quickcheck(outside_interval_unchanged as fn(u64, u32, u32) -> TestResult);
    }

    #[test]
    fn reverse_closed_interval_is_inverse() {
        fn rev_rev(x: u64, a: u32, b: u32) -> TestResult {
            if a >= BITS || b >= BITS || a >= b {
                return TestResult::discard();
            }
            TestResult::from_bool(
                x == reverse_closed_interval(reverse_closed_interval(x, a, b), a, b),
            )
        }
        QuickCheck::new()
            .tests(25_000)
            .max_tests(1_000_000)
            .quickcheck(rev_rev as fn(u64, u32, u32) -> TestResult);
    }
}
