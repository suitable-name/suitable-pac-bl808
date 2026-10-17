#[repr(C)]
#[doc = "AES Encryption Regions for Flash"]
#[doc(alias = "AES_REGION")]
pub struct AesRegion {
    key: [Key; 8],
    iv: [Iv; 4],
    start: Start,
    end: End,
}
impl AesRegion {
    #[doc = "0x00..0x20 - AES Key Word"]
    #[inline(always)]
    pub const fn key(&self, n: usize) -> &Key {
        &self.key[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - AES Key Word"]
    #[inline(always)]
    pub fn key_iter(&self) -> impl Iterator<Item = &Key> {
        self.key.iter()
    }
    #[doc = "0x20..0x30 - AES Initialization Vector Word"]
    #[inline(always)]
    pub const fn iv(&self, n: usize) -> &Iv {
        &self.iv[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x30 - AES Initialization Vector Word"]
    #[inline(always)]
    pub fn iv_iter(&self) -> impl Iterator<Item = &Iv> {
        self.iv.iter()
    }
    #[doc = "0x30 - Region Start Address and Config"]
    #[inline(always)]
    pub const fn start(&self) -> &Start {
        &self.start
    }
    #[doc = "0x34 - Region End Address"]
    #[inline(always)]
    pub const fn end(&self) -> &End {
        &self.end
    }
}
#[doc = "KEY (rw) register accessor: AES Key Word\n\nYou can [`read`](crate::Reg::read) this register and get [`key::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key`] module"]
#[doc(alias = "KEY")]
pub type Key = crate::Reg<key::KeySpec>;
#[doc = "AES Key Word"]
pub mod key;
#[doc = "IV (rw) register accessor: AES Initialization Vector Word\n\nYou can [`read`](crate::Reg::read) this register and get [`iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv`] module"]
#[doc(alias = "IV")]
pub type Iv = crate::Reg<iv::IvSpec>;
#[doc = "AES Initialization Vector Word"]
pub mod iv;
#[doc = "START (rw) register accessor: Region Start Address and Config\n\nYou can [`read`](crate::Reg::read) this register and get [`start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`] module"]
#[doc(alias = "START")]
pub type Start = crate::Reg<start::StartSpec>;
#[doc = "Region Start Address and Config"]
pub mod start;
#[doc = "END (rw) register accessor: Region End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@end`] module"]
#[doc(alias = "END")]
pub type End = crate::Reg<end::EndSpec>;
#[doc = "Region End Address"]
pub mod end;
