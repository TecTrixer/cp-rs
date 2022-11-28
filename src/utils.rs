#[doc(inline)]
// Library made by Boiethios, Licensed with Apache-2.0
/// Library for converting numbers to different radices
///
/// # Example
///
/// ```
/// use crate::cp_rs::utils::*;
///
/// let n: usize = 10;
/// assert!(radix(n, 2).to_string() == "1010");
/// ```
pub use radix_fmt::{radix, Radix};

#[doc(inline)]
/// Library for computing the md5 hash
///
/// # Example
/// 
/// ```
/// use crate::cp_rs::utils::*;
///
/// let hash = md5::compute("pqrstuv1048970");
/// assert!(format!("{:?}", hash) == "000006136ef2ff3b291c85725f17325c");
/// ```
pub use md5;

#[doc(inline)]
/// Macro for memoization / caching of recursive functions
///
/// # Example
///
/// ```
/// use crate::cp_rs::utils::*;
///
/// #[cached]
/// fn fib(n: usize) -> usize {
///     if n <= 1 { 1 } else { fib(n - 1) + fib(n - 2) }
/// }
///
/// assert!(fib(5) == 8)
/// ```
pub use cached::proc_macro::cached;
