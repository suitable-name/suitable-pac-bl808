#[doc = "Register `IRRX_DATA_WORD0` reader"]
pub type R = crate::R<IrrxDataWord0Spec>;
#[doc = "Field `sts_irrx_data_word0` reader - RX data word 0 bits \\[31:0\\]"]
pub type StsIrrxDataWord0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RX data word 0 bits \\[31:0\\]"]
    #[inline(always)]
    pub fn sts_irrx_data_word0(&self) -> StsIrrxDataWord0R {
        StsIrrxDataWord0R::new(self.bits)
    }
}
#[doc = "IRRX Received Data Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_data_word0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrrxDataWord0Spec;
impl crate::RegisterSpec for IrrxDataWord0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irrx_data_word0::R`](R) reader structure"]
impl crate::Readable for IrrxDataWord0Spec {}
#[doc = "`reset()` method sets IRRX_DATA_WORD0 to value 0"]
impl crate::Resettable for IrrxDataWord0Spec {}
