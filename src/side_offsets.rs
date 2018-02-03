// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A group of side offsets, which correspond to top/left/bottom/right for borders, padding,
//! and margins in CSS.

use super::UnknownUnit;
use length::Length;
use num::Zero;
use std::fmt;
use std::ops::Add;
use std::marker::PhantomData;

/// A group of side offsets, which correspond to top/left/bottom/right for borders, padding,
/// and margins in CSS, optionally tagged with a unit.
define_matrix! {
    pub struct TypedSideOffsets2D<T, U> {
        pub top: T,
        pub right: T,
        pub bottom: T,
        pub left: T,
    }
}

impl<T: fmt::Debug, U> fmt::Debug for TypedSideOffsets2D<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({:?},{:?},{:?},{:?})",
            self.top, self.right, self.bottom, self.left
        )
    }
}

/// The default side offset type with no unit.
pub type SideOffsets2D<T> = TypedSideOffsets2D<T, UnknownUnit>;

impl<T: Copy, U> TypedSideOffsets2D<T, U> {
    /// Constructor taking a scalar value or `Length` for each side.
    pub fn new<N: Into<Length<T, U>>>(top: N, right: N, bottom: N, left: N) -> Self {
        TypedSideOffsets2D {
            top: top.into().get(),
            right: right.into().get(),
            bottom: bottom.into().get(),
            left: left.into().get(),
            _unit: PhantomData,
        }
    }

    /// Access self.top as a typed Length instead of a scalar value.
    pub fn top_typed(&self) -> Length<T, U> {
        Length::new(self.top)
    }

    /// Access self.right as a typed Length instead of a scalar value.
    pub fn right_typed(&self) -> Length<T, U> {
        Length::new(self.right)
    }

    /// Access self.bottom as a typed Length instead of a scalar value.
    pub fn bottom_typed(&self) -> Length<T, U> {
        Length::new(self.bottom)
    }

    /// Access self.left as a typed Length instead of a scalar value.
    pub fn left_typed(&self) -> Length<T, U> {
        Length::new(self.left)
    }

    /// Constructor setting the same value to all sides, taking a scalar value or `Length`.
    pub fn new_all_same<N: Into<Length<T, U>>>(all: N) -> Self {
        let all = all.into().get();
        TypedSideOffsets2D::new(all, all, all, all)
    }
}

impl<T, U> TypedSideOffsets2D<T, U>
where
    T: Add<T, Output = T> + Copy,
{
    pub fn horizontal(&self) -> T {
        self.left + self.right
    }

    pub fn vertical(&self) -> T {
        self.top + self.bottom
    }

    pub fn horizontal_typed(&self) -> Length<T, U> {
        Length::new(self.horizontal())
    }

    pub fn vertical_typed(&self) -> Length<T, U> {
        Length::new(self.vertical())
    }
}

impl<T, U> Add for TypedSideOffsets2D<T, U>
where
    T: Copy + Add<T, Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        TypedSideOffsets2D::new(
            self.top + other.top,
            self.right + other.right,
            self.bottom + other.bottom,
            self.left + other.left,
        )
    }
}

impl<T: Copy + Zero, U> TypedSideOffsets2D<T, U> {
    /// Constructor, setting all sides to zero.
    pub fn zero() -> Self {
        TypedSideOffsets2D::new(T::zero(), T::zero(), T::zero(), T::zero())
    }
}
