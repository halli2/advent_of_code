#![allow(clippy::module_name_repetitions)]

use std::ops::{Index, IndexMut};

pub struct Vec2D<T> {
    pub inner: Vec<T>,
    stride: usize,
}

pub trait FastIndex<Idx: ?Sized> {
    type Output;
    fn get_unchecked_mut(&mut self, x: Idx, y: Idx) -> &mut Self::Output;
}

pub struct Array2D<const N: usize, T> {
    pub inner: [T; N],
    stride: usize,
}

impl<const N: usize, T> Array2D<N, T> {
    pub const fn new(inner: [T; N], stride: usize) -> Self {
        Self { inner, stride }
    }
}

impl<T> Vec2D<T> {
    pub const fn new(stride: usize) -> Self {
        Self {
            inner: Vec::new(),
            stride,
        }
    }
}

macro_rules! index {
    {$for:ident $ty:ident} => {
        impl<T> Index<($ty, $ty)> for $for<T> {
            type Output = T;

            fn index(&self, index: ($ty, $ty)) -> &Self::Output {
                &self.inner[(index.0 as usize * self.stride + index.1 as usize)]
            }
        }


        impl<T> IndexMut<($ty, $ty)> for $for<T> {
            fn index_mut(&mut self, index: ($ty, $ty)) -> &mut Self::Output {
                &mut self.inner[(index.0 as usize * self.stride + index.1 as usize)]
            }
        }


        impl<T> FastIndex<$ty> for $for<T> {
            type Output = T;

            #[inline(always)]
            fn get_unchecked_mut(&mut self, x: $ty, y: $ty) -> &mut Self::Output {
                unsafe { self.inner.get_unchecked_mut(x as usize * self.stride + y as usize) }
            }
        }
    }
}

macro_rules! const_index {
    {$for:ident $ty:ident} => {
        impl<const N: usize, T> Index<($ty, $ty)> for $for<N, T> {
            type Output = T;

            fn index(&self, index: ($ty, $ty)) -> &Self::Output {
                &self.inner[(index.0 as usize * self.stride + index.1 as usize)]
            }
        }


        impl<const N: usize, T> IndexMut<($ty, $ty)> for $for<N, T> {
            fn index_mut(&mut self, index: ($ty, $ty)) -> &mut Self::Output {
                &mut self.inner[(index.0 as usize * self.stride + index.1 as usize)]
            }
        }

        impl<const N: usize, T> FastIndex<$ty> for $for<N, T> {
            type Output = T;

            #[inline(always)]
            fn get_unchecked_mut(&mut self, x: $ty, y: $ty) -> &mut Self::Output {
                unsafe { self.inner.get_unchecked_mut(x as usize * self.stride + y as usize) }
            }
        }
    }
}

index! {Vec2D usize}
index! {Vec2D u32}
index! {Vec2D u8}
const_index! {Array2D usize}
const_index! {Array2D u32}
const_index! {Array2D u8}
