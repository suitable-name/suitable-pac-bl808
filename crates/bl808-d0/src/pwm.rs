#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    grp: (),
}
impl RegisterBlock {
    #[doc = "0x00..0x60 - PWM Group (Multi-Channel Block)"]
    #[inline(always)]
    pub const fn grp(&self, n: usize) -> &Grp {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x60 - PWM Group (Multi-Channel Block)"]
    #[inline(always)]
    pub fn grp_iter(&self) -> impl Iterator<Item = &Grp> {
        (0..2).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64 * n).cast() })
    }
}
#[doc = "PWM Group (Multi-Channel Block)"]
pub use self::grp::Grp;
#[doc = r"Cluster"]
#[doc = "PWM Group (Multi-Channel Block)"]
pub mod grp;
