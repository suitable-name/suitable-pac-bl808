#[doc = "Register `TZC_PSRAMA_TZSRG_R1` reader"]
pub type R = crate::R<TzcPsramaTzsrgR1Spec>;
#[doc = "Field `tzc_psrama_tzsrg_r1_end` reader - "]
pub type TzcPsramaTzsrgR1EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_psrama_tzsrg_r1_start` reader - "]
pub type TzcPsramaTzsrgR1StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r1_end(&self) -> TzcPsramaTzsrgR1EndR {
        TzcPsramaTzsrgR1EndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r1_start(&self) -> TzcPsramaTzsrgR1StartR {
        TzcPsramaTzsrgR1StartR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "PSRAM-A TZSRG Region 1 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psrama_tzsrg_r1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcPsramaTzsrgR1Spec;
impl crate::RegisterSpec for TzcPsramaTzsrgR1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_psrama_tzsrg_r1::R`](R) reader structure"]
impl crate::Readable for TzcPsramaTzsrgR1Spec {}
#[doc = "`reset()` method sets TZC_PSRAMA_TZSRG_R1 to value 0"]
impl crate::Resettable for TzcPsramaTzsrgR1Spec {}
