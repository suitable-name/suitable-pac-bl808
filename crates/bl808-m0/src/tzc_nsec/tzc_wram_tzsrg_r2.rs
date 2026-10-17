#[doc = "Register `TZC_WRAM_TZSRG_R2` reader"]
pub type R = crate::R<TzcWramTzsrgR2Spec>;
#[doc = "Field `tzc_wram_tzsrg_r2_end` reader - "]
pub type TzcWramTzsrgR2EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_wram_tzsrg_r2_start` reader - "]
pub type TzcWramTzsrgR2StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r2_end(&self) -> TzcWramTzsrgR2EndR {
        TzcWramTzsrgR2EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r2_start(&self) -> TzcWramTzsrgR2StartR {
        TzcWramTzsrgR2StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "WRAM TZSRG Region 2 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_wram_tzsrg_r2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcWramTzsrgR2Spec;
impl crate::RegisterSpec for TzcWramTzsrgR2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_wram_tzsrg_r2::R`](R) reader structure"]
impl crate::Readable for TzcWramTzsrgR2Spec {}
#[doc = "`reset()` method sets TZC_WRAM_TZSRG_R2 to value 0"]
impl crate::Resettable for TzcWramTzsrgR2Spec {}
