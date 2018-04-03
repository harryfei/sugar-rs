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

mod result_ext;
mod hashmap_ext;
mod collections_macros;
mod primitive_syntax;

pub use result_ext::SResultExt;
pub use hashmap_ext::SHashMapExt;
pub use collections_macros::*;
pub use primitive_syntax::*;
