
use std::mem::MaybeUninit;

/// Concatenates two arrays at compile time
///
/// Kind of a hack, so it should be replaced as soon as this function is available in stdlib
pub const fn const_concat<T, const N: usize, const M: usize>(
    arr1: &[T; N],
    arr2: &[T; M],
) -> [T; N + M] {
    unsafe {
        let mut combined: [T; N + M] = MaybeUninit::uninit().assume_init();
        std::ptr::copy_nonoverlapping(arr1.as_ptr(), combined.as_mut_ptr(), N);
        std::ptr::copy_nonoverlapping(arr2.as_ptr(), combined.as_mut_ptr().add(N), M);
        combined
    }

}
