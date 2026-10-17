#[doc = "Register `TZC_VRAM_TZSRG_R0` reader"]
pub type R = crate::R<TzcVramTzsrgR0Spec>;
#[doc = "Register `TZC_VRAM_TZSRG_R0` writer"]
pub type W = crate::W<TzcVramTzsrgR0Spec>;
#[doc = "Field `tzc_vram_tzsrg_r0_end` reader - End address boundary (10-bit offset)"]
pub type TzcVramTzsrgR0EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_vram_tzsrg_r0_end` writer - End address boundary (10-bit offset)"]
pub type TzcVramTzsrgR0EndW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `tzc_vram_tzsrg_r0_start` reader - Start address boundary (10-bit offset)"]
pub type TzcVramTzsrgR0StartR = crate::FieldReader<u16>;
#[doc = "Field `tzc_vram_tzsrg_r0_start` writer - Start address boundary (10-bit offset)"]
pub type TzcVramTzsrgR0StartW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - End address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r0_end(&self) -> TzcVramTzsrgR0EndR {
        TzcVramTzsrgR0EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Start address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r0_start(&self) -> TzcVramTzsrgR0StartR {
        TzcVramTzsrgR0StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - End address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r0_end(&mut self) -> TzcVramTzsrgR0EndW<'_, TzcVramTzsrgR0Spec> {
        TzcVramTzsrgR0EndW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Start address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r0_start(&mut self) -> TzcVramTzsrgR0StartW<'_, TzcVramTzsrgR0Spec> {
        TzcVramTzsrgR0StartW::new(self, 16)
    }
}
#[doc = "VRAM Security Region 0 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_vram_tzsrg_r0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_vram_tzsrg_r0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcVramTzsrgR0Spec;
impl crate::RegisterSpec for TzcVramTzsrgR0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_vram_tzsrg_r0::R`](R) reader structure"]
impl crate::Readable for TzcVramTzsrgR0Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_vram_tzsrg_r0::W`](W) writer structure"]
impl crate::Writable for TzcVramTzsrgR0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_VRAM_TZSRG_R0 to value 0x03ff"]
impl crate::Resettable for TzcVramTzsrgR0Spec {
    const RESET_VALUE: u32 = 0x03ff;
}
