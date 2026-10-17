#[doc = "Register `TZC_PSRAMA_TZSRG_R2` reader"]
pub type R = crate::R<TzcPsramaTzsrgR2Spec>;
#[doc = "Field `tzc_psrama_tzsrg_r2_end` reader - "]
pub type TzcPsramaTzsrgR2EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_psrama_tzsrg_r2_start` reader - "]
pub type TzcPsramaTzsrgR2StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r2_end(&self) -> TzcPsramaTzsrgR2EndR {
        TzcPsramaTzsrgR2EndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r2_start(&self) -> TzcPsramaTzsrgR2StartR {
        TzcPsramaTzsrgR2StartR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "PSRAM-A TZSRG Region 2 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psrama_tzsrg_r2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcPsramaTzsrgR2Spec;
impl crate::RegisterSpec for TzcPsramaTzsrgR2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_psrama_tzsrg_r2::R`](R) reader structure"]
impl crate::Readable for TzcPsramaTzsrgR2Spec {}
#[doc = "`reset()` method sets TZC_PSRAMA_TZSRG_R2 to value 0"]
impl crate::Resettable for TzcPsramaTzsrgR2Spec {}
