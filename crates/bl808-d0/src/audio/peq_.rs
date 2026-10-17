#[repr(C)]
#[doc = "Parametric Equalizer Bank %s: Sets the IIR biquad coefficients for the audio DSP chain"]
#[doc(alias = "peq_")]
pub struct Peq_ {
    b0: B0,
    b1: B1,
    b2: B2,
    a1: A1,
    a2: A2,
}
impl Peq_ {
    #[doc = "0x00 - PEQ Coefficient B0 (Numerator z0)"]
    #[inline(always)]
    pub const fn b0(&self) -> &B0 {
        &self.b0
    }
    #[doc = "0x04 - PEQ Coefficient B1 (Numerator z-1)"]
    #[inline(always)]
    pub const fn b1(&self) -> &B1 {
        &self.b1
    }
    #[doc = "0x08 - PEQ Coefficient B2 (Numerator z-2)"]
    #[inline(always)]
    pub const fn b2(&self) -> &B2 {
        &self.b2
    }
    #[doc = "0x0c - PEQ Coefficient A1 (Denominator z-1)"]
    #[inline(always)]
    pub const fn a1(&self) -> &A1 {
        &self.a1
    }
    #[doc = "0x10 - PEQ Coefficient A2 (Denominator z-2)"]
    #[inline(always)]
    pub const fn a2(&self) -> &A2 {
        &self.a2
    }
}
#[doc = "b0 (rw) register accessor: PEQ Coefficient B0 (Numerator z0)\n\nYou can [`read`](crate::Reg::read) this register and get [`b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0`] module"]
#[doc(alias = "b0")]
pub type B0 = crate::Reg<b0::B0Spec>;
#[doc = "PEQ Coefficient B0 (Numerator z0)"]
pub mod b0;
#[doc = "b1 (rw) register accessor: PEQ Coefficient B1 (Numerator z-1)\n\nYou can [`read`](crate::Reg::read) this register and get [`b1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b1`] module"]
#[doc(alias = "b1")]
pub type B1 = crate::Reg<b1::B1Spec>;
#[doc = "PEQ Coefficient B1 (Numerator z-1)"]
pub mod b1;
#[doc = "b2 (rw) register accessor: PEQ Coefficient B2 (Numerator z-2)\n\nYou can [`read`](crate::Reg::read) this register and get [`b2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b2`] module"]
#[doc(alias = "b2")]
pub type B2 = crate::Reg<b2::B2Spec>;
#[doc = "PEQ Coefficient B2 (Numerator z-2)"]
pub mod b2;
#[doc = "a1 (rw) register accessor: PEQ Coefficient A1 (Denominator z-1)\n\nYou can [`read`](crate::Reg::read) this register and get [`a1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a1`] module"]
#[doc(alias = "a1")]
pub type A1 = crate::Reg<a1::A1Spec>;
#[doc = "PEQ Coefficient A1 (Denominator z-1)"]
pub mod a1;
#[doc = "a2 (rw) register accessor: PEQ Coefficient A2 (Denominator z-2)\n\nYou can [`read`](crate::Reg::read) this register and get [`a2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a2`] module"]
#[doc(alias = "a2")]
pub type A2 = crate::Reg<a2::A2Spec>;
#[doc = "PEQ Coefficient A2 (Denominator z-2)"]
pub mod a2;
