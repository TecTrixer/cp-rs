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
pub use radix_fmt::radix;

#[doc(inline)]
pub use radix_fmt::Radix;

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

#[doc(inline)]
/// Struct for handling complex numbers
///
/// # Example
///
/// ```
/// use crate::cp_rs::utils::*;
///
/// // c = 1 + 2i (with precision 53)
/// let c = Complex::with_val(53, (1, 2));
/// assert!(*c.real() == 1.0);
/// assert!(*c.imag() == 2.0);
/// ```
pub use rug::Complex;

#[doc(inline)]
/// Struct for handling big integers
///
/// # Example
///
/// ```
/// use crate::cp_rs::utils::*;
///
/// let mut num = Integer::from(u128::MAX);
/// num *= 10;
/// assert!(num % 10 == 0);
/// ```
pub use rug::Integer;

#[doc(inline)]
/// Struct for handling arbitrary precision floats
///
/// # Example
///
/// ```
/// use crate::cp_rs::utils::*;
///
/// let float = Float::with_val(53, 4);
/// let sqrt  = float.sqrt();
/// assert!(sqrt == 2.0);
/// ```
pub use rug::Float;

#[doc(inline)]
/// Struct for handling arbitrary precision floats
///
/// # Example
///
/// ```
/// use crate::cp_rs::utils::*;
///
/// let mut rational = Rational::from((-2, 3));
/// rational *= 3;
/// assert!(rational == -2);
/// ```
pub use rug::Rational;
