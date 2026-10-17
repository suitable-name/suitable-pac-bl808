#[doc = "Register `TZC_VRAM_TZSRG_R0` reader"]
pub type R = crate::R<TzcVramTzsrgR0Spec>;
#[doc = "Field `tzc_vram_tzsrg_r0_end` reader - "]
pub type TzcVramTzsrgR0EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_vram_tzsrg_r0_start` reader - "]
pub type TzcVramTzsrgR0StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r0_end(&self) -> TzcVramTzsrgR0EndR {
        TzcVramTzsrgR0EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r0_start(&self) -> TzcVramTzsrgR0StartR {
        TzcVramTzsrgR0StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "VRAM TZSRG Region 0 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_vram_tzsrg_r0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcVramTzsrgR0Spec;
impl crate::RegisterSpec for TzcVramTzsrgR0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_vram_tzsrg_r0::R`](R) reader structure"]
impl crate::Readable for TzcVramTzsrgR0Spec {}
#[doc = "`reset()` method sets TZC_VRAM_TZSRG_R0 to value 0"]
impl crate::Resettable for TzcVramTzsrgR0Spec {}
