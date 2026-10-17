#[doc = "Register `TZC_SF_TZSRG_R2` reader"]
pub type R = crate::R<TzcSfTzsrgR2Spec>;
#[doc = "Field `tzc_sf_tzsrg_r2_end` reader - "]
pub type TzcSfTzsrgR2EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_sf_tzsrg_r2_start` reader - "]
pub type TzcSfTzsrgR2StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_end(&self) -> TzcSfTzsrgR2EndR {
        TzcSfTzsrgR2EndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_start(&self) -> TzcSfTzsrgR2StartR {
        TzcSfTzsrgR2StartR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "SF TZSRG Region 2 Start/End (Bits \\[18:0\\])\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_r2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcSfTzsrgR2Spec;
impl crate::RegisterSpec for TzcSfTzsrgR2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_sf_tzsrg_r2::R`](R) reader structure"]
impl crate::Readable for TzcSfTzsrgR2Spec {}
#[doc = "`reset()` method sets TZC_SF_TZSRG_R2 to value 0"]
impl crate::Resettable for TzcSfTzsrgR2Spec {}
