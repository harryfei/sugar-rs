//! Syntax sugar to make your Rust life more sweet.
//!
//! # Usage
//! ```rust
//! use sugar::*;
//! ```
//! # Sugars
//!
//! ## Collections construct
//! ```
//! use sugar::*;
//!
//! // vec construction
//! let v = vec![1, 2, 3];
//!
//! // vec of boxed value
//! let vb = vec_box![1, 2, 3];
//!
//! // hashmap construction
//! let hm = hashmap!{
//!     1 => 2,
//!     2 => 3,
//!     3 => 4,
//! };
//!
//! // list comprehension
//! let vb2 = c![Box::new(i), for i in 1..4];
//! assert_eq!(vb, vb2);
//!
//! // hashmap comprehension
//! let hm2 = c![i => i + 1, for i in 1..4];
//! assert_eq!(hm, hm2);
//! ```
//!
//! * `vec_box!` is reexported from [`vec_box`](https://docs.rs/vec_box)
//! * `hashmap!` and related macros are reexported from [`maplit`](https://docs.rs/maplit) crate
//! * `c!` list comprehension macro is reexported from [`cute`](https://docs.rs/cute) crate
//!
//! ## Hashmap: clone a value of speical key.
//!
//! ```
//! use sugar::*;
//!
//! let hm = hashmap! {
//!     1 => "1".to_owned(),
//!     2 => "2".to_owned(),
//! };
//!
//! let s = {
//!     hm.get_clone(&1)
//! };
//!
//! assert_eq!(s.unwrap(), "1".to_owned());
//!
//! ```
//!
//! ## Result: ignore Result's OK value
//! ```
//! use sugar::SResultExt;
//!
//! fn do_work1() -> Result<i32, String> {
//!     Ok(1)
//! }
//!
//! fn do_work2() -> Result<(), String> {
//!     // I don't care about the result's Ok value
//!     do_work1()
//!         .drop_value()
//! }
//!
//! do_work2();
//! ```
//!
//! ## max! and min! macros
//! ```
//! use sugar::*;
//!
//! assert_eq!(max!(3, 1, 2, 7, 0), 7);
//! assert_eq!(min!(3, 1, 2, 7, 0), 0);
//! ```
//! * `max!` and `min!` reexported from [`min_max_macros`](https://docs.rs/min_max_macros)
//!
//!
//! ## Chained comparison
//! ```
//! use sugar::*;
//!
//! fn return_2() -> i32 {
//!     return 2;
//! }
//!
//! assert_eq!(cmp!(1, < 2, < 3), true);
//! assert_eq!(cmp!(1, <= 2, <= 3), true);
//! assert_eq!(cmp!(1, <= 2, <= 3, != 4), true);
//!
//! // return_2 will not be called
//! assert_eq!(cmp!(2, < 1, < 3, < return_2()), false);
//!
//! // return_2 will be called once
//! assert_eq!(cmp!(1, < return_2(), <= 2), true);
//! ```

#[macro_use]
#[allow(unused_imports)]
extern crate maplit;

#[macro_use]
#[allow(unused_imports)]
extern crate cute;

#[macro_use]
#[allow(unused_imports)]
extern crate vec_box;

#[macro_use]
#[allow(unused_imports)]
extern crate min_max_macros;

mod collections_macros;
mod hashmap_ext;
mod number_ext;
mod primitive_syntax;
mod result_ext;

pub use collections_macros::*;
pub use hashmap_ext::SHashMapExt;
pub use number_ext::SNumExt;
pub use primitive_syntax::*;
pub use result_ext::SResultExt;
