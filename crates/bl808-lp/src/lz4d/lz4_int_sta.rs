#[doc = "Register `LZ4_INT_STA` reader"]
pub type R = crate::R<Lz4IntStaSpec>;
#[doc = "Field `lz4_done_sta` reader - Decompression done status"]
pub type Lz4DoneStaR = crate::BitReader;
#[doc = "Field `lz4_err_sta` reader - Decompression error status"]
pub type Lz4ErrStaR = crate::BitReader;
#[doc = "Field `lz4_dst_int_sta` reader - Destination-specific interrupt status"]
pub type Lz4DstIntStaR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Decompression done status"]
    #[inline(always)]
    pub fn lz4_done_sta(&self) -> Lz4DoneStaR {
        Lz4DoneStaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Decompression error status"]
    #[inline(always)]
    pub fn lz4_err_sta(&self) -> Lz4ErrStaR {
        Lz4ErrStaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 10:15 - Destination-specific interrupt status"]
    #[inline(always)]
    pub fn lz4_dst_int_sta(&self) -> Lz4DstIntStaR {
        Lz4DstIntStaR::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
#[doc = "LZ4 Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lz4_int_sta::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lz4IntStaSpec;
impl crate::RegisterSpec for Lz4IntStaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lz4_int_sta::R`](R) reader structure"]
impl crate::Readable for Lz4IntStaSpec {}
#[doc = "`reset()` method sets LZ4_INT_STA to value 0"]
impl crate::Resettable for Lz4IntStaSpec {}
