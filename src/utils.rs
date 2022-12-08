use core::slice::memchr;

pub trait ByteExt {
    fn memchr(&self, needle: u8) -> usize;
}

impl ByteExt for &[u8] {
    #[inline]
    fn memchr(&self, needle: u8) -> usize {
        unsafe { memchr::memchr(needle, self).unwrap_unchecked() }
    }
}

// use std::{
//     iter,
//     ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
// };
// pub trait Integer:
//     Copy
//     + From<u8>
//     + AddAssign
//     + Add<Output = Self>
//     + SubAssign
//     + Sub<Output = Self>
//     + MulAssign
//     + Mul<Output = Self>
//     + Eq
//     + PartialEq
//     + Ord
//     + PartialOrd
//     + Default
//     + iter::Sum
//     + iter::Product
// {
// }
// impl Integer for u8 {}
// impl Integer for u16 {}
// impl Integer for u32 {}
// impl Integer for u64 {}
// impl Integer for i32 {}
// impl Integer for i64 {}
// impl Integer for usize {}
// impl Integer for isize {}

// // TODO: Find out why this is so slow? Much slower than manual in day07
// // even without unwrap checking..
// #[inline(always)]
// pub fn atoi_with_first_byte<T: Integer, const MIN: usize, const MAX: usize>(
//     iter: &mut dyn Iterator<Item = &u8>,
//     byte_1: &u8,
// ) -> T {
//     unsafe {
//         // let b = iter.next().unwrap_unchecked();
//         let mut value = T::from(byte_1 & 0x0f);
//         for _ in 1..MIN {
//             let b = iter.next().unwrap_unchecked();
//             let next = T::from(b & 0x0f);
//             value = value * T::from(10_u8) + next;
//         }
//         for _ in MIN..MAX {
//             let b = iter.next().unwrap_unchecked();
//             // let next = b.wrapping_sub(b'0');
//             // if next < 10 {
//             if *b == b' ' {
//                 return value;
//             }
//             let next = T::from(b & 0x0f);
//             value = value * T::from(10_u8) + next;
//         }
//         value
//     }
// }
