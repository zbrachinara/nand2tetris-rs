
use std::mem::MaybeUninit;

/// Concatenates two arrays at compile time
///
/// Kind of a hack, so it should be replaced as soon as this function is available in stdlib
pub const fn const_concat<T: Clone, const N: usize, const M: usize>(
    arr1: [T; N],
    arr2: [T; M],
) -> [T; N + M] {
    let mut combined = unsafe { [MaybeUninit::uninit().assume_init(); N + M] };
    let mut i = 0;
    while i < arr1.len() {
        combined[i] = arr1[i].clone();
        i += 1;
    }
    let mut j = 0;
    while j < arr2.len() {
        combined[i + j] = arr2[j].clone();
        j += 1;
    }
    combined
}
