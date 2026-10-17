#[doc = "Register `IRRX_DATA_WORD1` reader"]
pub type R = crate::R<IrrxDataWord1Spec>;
#[doc = "Field `sts_irrx_data_word1` reader - RX data word 1 bits \\[63:32\\]"]
pub type StsIrrxDataWord1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RX data word 1 bits \\[63:32\\]"]
    #[inline(always)]
    pub fn sts_irrx_data_word1(&self) -> StsIrrxDataWord1R {
        StsIrrxDataWord1R::new(self.bits)
    }
}
#[doc = "IRRX Received Data Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_data_word1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrrxDataWord1Spec;
impl crate::RegisterSpec for IrrxDataWord1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irrx_data_word1::R`](R) reader structure"]
impl crate::Readable for IrrxDataWord1Spec {}
#[doc = "`reset()` method sets IRRX_DATA_WORD1 to value 0"]
impl crate::Resettable for IrrxDataWord1Spec {}
