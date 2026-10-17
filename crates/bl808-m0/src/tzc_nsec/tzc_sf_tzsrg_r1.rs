#[doc = "Register `TZC_SF_TZSRG_R1` reader"]
pub type R = crate::R<TzcSfTzsrgR1Spec>;
#[doc = "Field `tzc_sf_tzsrg_r1_end` reader - "]
pub type TzcSfTzsrgR1EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_sf_tzsrg_r1_start` reader - "]
pub type TzcSfTzsrgR1StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_end(&self) -> TzcSfTzsrgR1EndR {
        TzcSfTzsrgR1EndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_start(&self) -> TzcSfTzsrgR1StartR {
        TzcSfTzsrgR1StartR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "SF TZSRG Region 1 Start/End (Bits \\[18:0\\])\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_r1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcSfTzsrgR1Spec;
impl crate::RegisterSpec for TzcSfTzsrgR1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_sf_tzsrg_r1::R`](R) reader structure"]
impl crate::Readable for TzcSfTzsrgR1Spec {}
#[doc = "`reset()` method sets TZC_SF_TZSRG_R1 to value 0"]
impl crate::Resettable for TzcSfTzsrgR1Spec {}
