#[doc = "Register `TZC_PSRAMB_TZSRG_R0` reader"]
pub type R = crate::R<TzcPsrambTzsrgR0Spec>;
#[doc = "Field `tzc_psramb_tzsrg_r0_end` reader - "]
pub type TzcPsrambTzsrgR0EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_psramb_tzsrg_r0_start` reader - "]
pub type TzcPsrambTzsrgR0StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r0_end(&self) -> TzcPsrambTzsrgR0EndR {
        TzcPsrambTzsrgR0EndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r0_start(&self) -> TzcPsrambTzsrgR0StartR {
        TzcPsrambTzsrgR0StartR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "PSRAM-B TZSRG Region 0 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psramb_tzsrg_r0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcPsrambTzsrgR0Spec;
impl crate::RegisterSpec for TzcPsrambTzsrgR0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_psramb_tzsrg_r0::R`](R) reader structure"]
impl crate::Readable for TzcPsrambTzsrgR0Spec {}
#[doc = "`reset()` method sets TZC_PSRAMB_TZSRG_R0 to value 0"]
impl crate::Resettable for TzcPsrambTzsrgR0Spec {}
