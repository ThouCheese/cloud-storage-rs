
#[cfg(feature = "openssl")]
mod openssl;
#[cfg(feature = "openssl")]
pub use self::openssl::*;

#[cfg(all(feature = "ring", not(feature = "openssl")))]
mod ring;
#[cfg(all(feature = "ring", not(feature = "openssl")))]
pub use self::ring::*;