//! Reproduced under MIT license from https://github.com/image-rs/imageproc/blob/908b9dcc257ba3016dccc151851dd7701f37ec47/src/map.rs#L10:
//!
//! The MIT License (MIT)
//!
//! Copyright (c) 2015 PistonDevelopers
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.

use image::{Pixel, Primitive, Rgb};

/// The type obtained by replacing the channel type of a given `Pixel` type.
/// The output type must have the same name of channels as the input type, or
/// several algorithms will produce incorrect results or panic.
pub trait WithChannel<C: Primitive>: Pixel {
    /// The new pixel type.
    type Pixel: Pixel<Subpixel = C>;
}

impl<T, U> WithChannel<U> for Rgb<T>
where
    T: Primitive,
    U: Primitive,
{
    type Pixel = Rgb<U>;
}
