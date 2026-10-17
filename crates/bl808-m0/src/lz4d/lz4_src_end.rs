#[doc = "Register `LZ4_SRC_END` reader"]
pub type R = crate::R<Lz4SrcEndSpec>;
#[doc = "Field `lz4_src_end` reader - Final source address after decompression"]
pub type Lz4SrcEndR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:25 - Final source address after decompression"]
    #[inline(always)]
    pub fn lz4_src_end(&self) -> Lz4SrcEndR {
        Lz4SrcEndR::new(self.bits & 0x03ff_ffff)
    }
}
#[doc = "Source End Address Status\n\nYou can [`read`](crate::Reg::read) this register and get [`lz4_src_end::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lz4SrcEndSpec;
impl crate::RegisterSpec for Lz4SrcEndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lz4_src_end::R`](R) reader structure"]
impl crate::Readable for Lz4SrcEndSpec {}
#[doc = "`reset()` method sets LZ4_SRC_END to value 0"]
impl crate::Resettable for Lz4SrcEndSpec {}
