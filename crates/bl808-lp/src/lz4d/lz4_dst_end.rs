#[doc = "Register `LZ4_DST_END` reader"]
pub type R = crate::R<Lz4DstEndSpec>;
#[doc = "Field `lz4_dst_end` reader - Final destination address after decompression"]
pub type Lz4DstEndR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:25 - Final destination address after decompression"]
    #[inline(always)]
    pub fn lz4_dst_end(&self) -> Lz4DstEndR {
        Lz4DstEndR::new(self.bits & 0x03ff_ffff)
    }
}
#[doc = "Destination End Address Status\n\nYou can [`read`](crate::Reg::read) this register and get [`lz4_dst_end::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lz4DstEndSpec;
impl crate::RegisterSpec for Lz4DstEndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lz4_dst_end::R`](R) reader structure"]
impl crate::Readable for Lz4DstEndSpec {}
#[doc = "`reset()` method sets LZ4_DST_END to value 0"]
impl crate::Resettable for Lz4DstEndSpec {}
