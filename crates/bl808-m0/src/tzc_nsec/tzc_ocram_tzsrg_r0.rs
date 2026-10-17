#[doc = "Register `TZC_OCRAM_TZSRG_R0` reader"]
pub type R = crate::R<TzcOcramTzsrgR0Spec>;
#[doc = "Field `tzc_ocram_tzsrg_r0_end` reader - "]
pub type TzcOcramTzsrgR0EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_ocram_tzsrg_r0_start` reader - "]
pub type TzcOcramTzsrgR0StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r0_end(&self) -> TzcOcramTzsrgR0EndR {
        TzcOcramTzsrgR0EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r0_start(&self) -> TzcOcramTzsrgR0StartR {
        TzcOcramTzsrgR0StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "OCRAM TZSRG Region 0 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_ocram_tzsrg_r0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcOcramTzsrgR0Spec;
impl crate::RegisterSpec for TzcOcramTzsrgR0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_ocram_tzsrg_r0::R`](R) reader structure"]
impl crate::Readable for TzcOcramTzsrgR0Spec {}
#[doc = "`reset()` method sets TZC_OCRAM_TZSRG_R0 to value 0"]
impl crate::Resettable for TzcOcramTzsrgR0Spec {}
