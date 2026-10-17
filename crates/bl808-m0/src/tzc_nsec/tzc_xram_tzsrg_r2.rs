#[doc = "Register `TZC_XRAM_TZSRG_R2` reader"]
pub type R = crate::R<TzcXramTzsrgR2Spec>;
#[doc = "Field `tzc_xram_tzsrg_r2_end` reader - "]
pub type TzcXramTzsrgR2EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_xram_tzsrg_r2_start` reader - "]
pub type TzcXramTzsrgR2StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r2_end(&self) -> TzcXramTzsrgR2EndR {
        TzcXramTzsrgR2EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r2_start(&self) -> TzcXramTzsrgR2StartR {
        TzcXramTzsrgR2StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "XRAM TZSRG Region 2 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_xram_tzsrg_r2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcXramTzsrgR2Spec;
impl crate::RegisterSpec for TzcXramTzsrgR2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_xram_tzsrg_r2::R`](R) reader structure"]
impl crate::Readable for TzcXramTzsrgR2Spec {}
#[doc = "`reset()` method sets TZC_XRAM_TZSRG_R2 to value 0"]
impl crate::Resettable for TzcXramTzsrgR2Spec {}
