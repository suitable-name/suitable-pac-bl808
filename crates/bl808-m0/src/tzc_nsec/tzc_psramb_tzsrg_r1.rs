#[doc = "Register `TZC_PSRAMB_TZSRG_R1` reader"]
pub type R = crate::R<TzcPsrambTzsrgR1Spec>;
#[doc = "Field `tzc_psramb_tzsrg_r1_end` reader - "]
pub type TzcPsrambTzsrgR1EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_psramb_tzsrg_r1_start` reader - "]
pub type TzcPsrambTzsrgR1StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r1_end(&self) -> TzcPsrambTzsrgR1EndR {
        TzcPsrambTzsrgR1EndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r1_start(&self) -> TzcPsrambTzsrgR1StartR {
        TzcPsrambTzsrgR1StartR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "PSRAM-B TZSRG Region 1 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psramb_tzsrg_r1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcPsrambTzsrgR1Spec;
impl crate::RegisterSpec for TzcPsrambTzsrgR1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_psramb_tzsrg_r1::R`](R) reader structure"]
impl crate::Readable for TzcPsrambTzsrgR1Spec {}
#[doc = "`reset()` method sets TZC_PSRAMB_TZSRG_R1 to value 0"]
impl crate::Resettable for TzcPsrambTzsrgR1Spec {}
