#[doc = "Register `TZC_SF_TZSRG_R3` reader"]
pub type R = crate::R<TzcSfTzsrgR3Spec>;
#[doc = "Field `tzc_sf_tzsrg_r3_end` reader - "]
pub type TzcSfTzsrgR3EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_sf_tzsrg_r3_start` reader - "]
pub type TzcSfTzsrgR3StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_end(&self) -> TzcSfTzsrgR3EndR {
        TzcSfTzsrgR3EndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_start(&self) -> TzcSfTzsrgR3StartR {
        TzcSfTzsrgR3StartR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "SF TZSRG Region 3 Start/End (Bits \\[18:0\\])\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_r3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcSfTzsrgR3Spec;
impl crate::RegisterSpec for TzcSfTzsrgR3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_sf_tzsrg_r3::R`](R) reader structure"]
impl crate::Readable for TzcSfTzsrgR3Spec {}
#[doc = "`reset()` method sets TZC_SF_TZSRG_R3 to value 0"]
impl crate::Resettable for TzcSfTzsrgR3Spec {}
