#[doc = "Register `TZC_VRAM_TZSRG_R2` reader"]
pub type R = crate::R<TzcVramTzsrgR2Spec>;
#[doc = "Register `TZC_VRAM_TZSRG_R2` writer"]
pub type W = crate::W<TzcVramTzsrgR2Spec>;
#[doc = "Field `tzc_vram_tzsrg_r2_end` reader - End address boundary (10-bit offset)"]
pub type TzcVramTzsrgR2EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_vram_tzsrg_r2_end` writer - End address boundary (10-bit offset)"]
pub type TzcVramTzsrgR2EndW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `tzc_vram_tzsrg_r2_start` reader - Start address boundary (10-bit offset)"]
pub type TzcVramTzsrgR2StartR = crate::FieldReader<u16>;
#[doc = "Field `tzc_vram_tzsrg_r2_start` writer - Start address boundary (10-bit offset)"]
pub type TzcVramTzsrgR2StartW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - End address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r2_end(&self) -> TzcVramTzsrgR2EndR {
        TzcVramTzsrgR2EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Start address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r2_start(&self) -> TzcVramTzsrgR2StartR {
        TzcVramTzsrgR2StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - End address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r2_end(&mut self) -> TzcVramTzsrgR2EndW<'_, TzcVramTzsrgR2Spec> {
        TzcVramTzsrgR2EndW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Start address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r2_start(&mut self) -> TzcVramTzsrgR2StartW<'_, TzcVramTzsrgR2Spec> {
        TzcVramTzsrgR2StartW::new(self, 16)
    }
}
#[doc = "VRAM Security Region 2 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_vram_tzsrg_r2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_vram_tzsrg_r2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcVramTzsrgR2Spec;
impl crate::RegisterSpec for TzcVramTzsrgR2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_vram_tzsrg_r2::R`](R) reader structure"]
impl crate::Readable for TzcVramTzsrgR2Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_vram_tzsrg_r2::W`](W) writer structure"]
impl crate::Writable for TzcVramTzsrgR2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_VRAM_TZSRG_R2 to value 0x03ff"]
impl crate::Resettable for TzcVramTzsrgR2Spec {
    const RESET_VALUE: u32 = 0x03ff;
}
