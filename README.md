```
error[E0277]: the trait bound `T: image::traits::Enlargeable` is not satisfied
  --> src\lib.rs:35:12
   |
35 | impl<T, U> WithChannel<U> for Rgb<T>
   |            ^^^^^^^^^^^^^^ the trait `image::traits::Enlargeable` is not implemented for `T`
   |
   = note: required because of the requirements on the impl of `Pixel` for `Rgb<T>`
note: required by a bound in `WithChannel`
  --> src\lib.rs:30:38
   |
30 | pub trait WithChannel<C: Primitive>: Pixel {
   |                                      ^^^^^ required by this bound in `WithChannel`
help: consider further restricting this bound
   |
37 |     T: Primitive + image::traits::Enlargeable,
   |                  ++++++++++++++++++++++++++++

```

NB: `image::traits::Enlargeable` is not `pub` outside the [scope](https://github.com/image-rs/image/blob/398b087ca8b97322f216a159e40414cdb408e29b/src/lib.rs#L257) of the image crate.