#[doc = "Register `TZC_VRAM_TZSRG_R2` reader"]
pub type R = crate::R<TzcVramTzsrgR2Spec>;
#[doc = "Field `tzc_vram_tzsrg_r2_end` reader - "]
pub type TzcVramTzsrgR2EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_vram_tzsrg_r2_start` reader - "]
pub type TzcVramTzsrgR2StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r2_end(&self) -> TzcVramTzsrgR2EndR {
        TzcVramTzsrgR2EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r2_start(&self) -> TzcVramTzsrgR2StartR {
        TzcVramTzsrgR2StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "VRAM TZSRG Region 2 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_vram_tzsrg_r2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcVramTzsrgR2Spec;
impl crate::RegisterSpec for TzcVramTzsrgR2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_vram_tzsrg_r2::R`](R) reader structure"]
impl crate::Readable for TzcVramTzsrgR2Spec {}
#[doc = "`reset()` method sets TZC_VRAM_TZSRG_R2 to value 0"]
impl crate::Resettable for TzcVramTzsrgR2Spec {}
