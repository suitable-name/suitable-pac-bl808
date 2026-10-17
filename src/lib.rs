#![no_std]

#[cfg(feature = "m0")]
pub mod m0 {
    pub use bl808_m0::*;
}

#[cfg(feature = "d0")]
pub mod d0 {
    pub use bl808_d0::*;
}

#[cfg(feature = "lp")]
pub mod lp {
    pub use bl808_lp::*;
}

// Re-export the contents of the selected sub-crate at root level
// when exactly one core feature is active.
#[cfg(all(feature = "m0", not(feature = "d0"), not(feature = "lp")))]
pub use bl808_m0::*;

#[cfg(all(feature = "d0", not(feature = "m0"), not(feature = "lp")))]
pub use bl808_d0::*;

#[cfg(all(feature = "lp", not(feature = "m0"), not(feature = "d0")))]
pub use bl808_lp::*;
