#[doc = "Register `IROM1_MISR_DATAOUT_1` reader"]
pub type R = crate::R<Irom1MisrDataout1Spec>;
#[doc = "Field `irom1_misr_dataout_1` reader - "]
pub type Irom1MisrDataout1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn irom1_misr_dataout_1(&self) -> Irom1MisrDataout1R {
        Irom1MisrDataout1R::new(self.bits)
    }
}
#[doc = "IROM1 MISR Data Output 1 (Test/Debug)\n\nYou can [`read`](crate::Reg::read) this register and get [`irom1_misr_dataout_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Irom1MisrDataout1Spec;
impl crate::RegisterSpec for Irom1MisrDataout1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irom1_misr_dataout_1::R`](R) reader structure"]
impl crate::Readable for Irom1MisrDataout1Spec {}
#[doc = "`reset()` method sets IROM1_MISR_DATAOUT_1 to value 0"]
impl crate::Resettable for Irom1MisrDataout1Spec {}
