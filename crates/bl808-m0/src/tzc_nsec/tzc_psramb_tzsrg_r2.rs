#[doc = "Register `TZC_PSRAMB_TZSRG_R2` reader"]
pub type R = crate::R<TzcPsrambTzsrgR2Spec>;
#[doc = "Field `tzc_psramb_tzsrg_r2_end` reader - "]
pub type TzcPsrambTzsrgR2EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_psramb_tzsrg_r2_start` reader - "]
pub type TzcPsrambTzsrgR2StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r2_end(&self) -> TzcPsrambTzsrgR2EndR {
        TzcPsrambTzsrgR2EndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r2_start(&self) -> TzcPsrambTzsrgR2StartR {
        TzcPsrambTzsrgR2StartR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "PSRAM-B TZSRG Region 2 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psramb_tzsrg_r2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcPsrambTzsrgR2Spec;
impl crate::RegisterSpec for TzcPsrambTzsrgR2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_psramb_tzsrg_r2::R`](R) reader structure"]
impl crate::Readable for TzcPsrambTzsrgR2Spec {}
#[doc = "`reset()` method sets TZC_PSRAMB_TZSRG_R2 to value 0"]
impl crate::Resettable for TzcPsrambTzsrgR2Spec {}
