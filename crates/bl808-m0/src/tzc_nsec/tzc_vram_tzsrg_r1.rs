#[doc = "Register `TZC_VRAM_TZSRG_R1` reader"]
pub type R = crate::R<TzcVramTzsrgR1Spec>;
#[doc = "Field `tzc_vram_tzsrg_r1_end` reader - "]
pub type TzcVramTzsrgR1EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_vram_tzsrg_r1_start` reader - "]
pub type TzcVramTzsrgR1StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r1_end(&self) -> TzcVramTzsrgR1EndR {
        TzcVramTzsrgR1EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r1_start(&self) -> TzcVramTzsrgR1StartR {
        TzcVramTzsrgR1StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "VRAM TZSRG Region 1 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_vram_tzsrg_r1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcVramTzsrgR1Spec;
impl crate::RegisterSpec for TzcVramTzsrgR1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_vram_tzsrg_r1::R`](R) reader structure"]
impl crate::Readable for TzcVramTzsrgR1Spec {}
#[doc = "`reset()` method sets TZC_VRAM_TZSRG_R1 to value 0"]
impl crate::Resettable for TzcVramTzsrgR1Spec {}
