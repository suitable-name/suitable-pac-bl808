#[doc = "Register `TZC_SF_TZSRG_R0` reader"]
pub type R = crate::R<TzcSfTzsrgR0Spec>;
#[doc = "Field `tzc_sf_tzsrg_r0_end` reader - "]
pub type TzcSfTzsrgR0EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_sf_tzsrg_r0_start` reader - "]
pub type TzcSfTzsrgR0StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r0_end(&self) -> TzcSfTzsrgR0EndR {
        TzcSfTzsrgR0EndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r0_start(&self) -> TzcSfTzsrgR0StartR {
        TzcSfTzsrgR0StartR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "SF TZSRG Region 0 Start/End (Bits \\[18:0\\])\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_r0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcSfTzsrgR0Spec;
impl crate::RegisterSpec for TzcSfTzsrgR0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_sf_tzsrg_r0::R`](R) reader structure"]
impl crate::Readable for TzcSfTzsrgR0Spec {}
#[doc = "`reset()` method sets TZC_SF_TZSRG_R0 to value 0"]
impl crate::Resettable for TzcSfTzsrgR0Spec {}
