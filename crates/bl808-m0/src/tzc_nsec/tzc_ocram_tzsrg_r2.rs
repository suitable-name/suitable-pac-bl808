#[doc = "Register `TZC_OCRAM_TZSRG_R2` reader"]
pub type R = crate::R<TzcOcramTzsrgR2Spec>;
#[doc = "Field `tzc_ocram_tzsrg_r2_end` reader - "]
pub type TzcOcramTzsrgR2EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_ocram_tzsrg_r2_start` reader - "]
pub type TzcOcramTzsrgR2StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r2_end(&self) -> TzcOcramTzsrgR2EndR {
        TzcOcramTzsrgR2EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r2_start(&self) -> TzcOcramTzsrgR2StartR {
        TzcOcramTzsrgR2StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "OCRAM TZSRG Region 2 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_ocram_tzsrg_r2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcOcramTzsrgR2Spec;
impl crate::RegisterSpec for TzcOcramTzsrgR2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_ocram_tzsrg_r2::R`](R) reader structure"]
impl crate::Readable for TzcOcramTzsrgR2Spec {}
#[doc = "`reset()` method sets TZC_OCRAM_TZSRG_R2 to value 0"]
impl crate::Resettable for TzcOcramTzsrgR2Spec {}
