#[doc = "Register `TZC_ROM_TZSRG_R2` reader"]
pub type R = crate::R<TzcRomTzsrgR2Spec>;
#[doc = "Register `TZC_ROM_TZSRG_R2` writer"]
pub type W = crate::W<TzcRomTzsrgR2Spec>;
#[doc = "Field `tzc_rom_tzsrg_r2_end` reader - End address boundary (10-bit offset)"]
pub type TzcRomTzsrgR2EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_rom_tzsrg_r2_end` writer - End address boundary (10-bit offset)"]
pub type TzcRomTzsrgR2EndW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `tzc_rom_tzsrg_r2_start` reader - Start address boundary (10-bit offset)"]
pub type TzcRomTzsrgR2StartR = crate::FieldReader<u16>;
#[doc = "Field `tzc_rom_tzsrg_r2_start` writer - Start address boundary (10-bit offset)"]
pub type TzcRomTzsrgR2StartW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - End address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r2_end(&self) -> TzcRomTzsrgR2EndR {
        TzcRomTzsrgR2EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Start address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r2_start(&self) -> TzcRomTzsrgR2StartR {
        TzcRomTzsrgR2StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - End address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r2_end(&mut self) -> TzcRomTzsrgR2EndW<'_, TzcRomTzsrgR2Spec> {
        TzcRomTzsrgR2EndW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Start address boundary (10-bit offset)"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r2_start(&mut self) -> TzcRomTzsrgR2StartW<'_, TzcRomTzsrgR2Spec> {
        TzcRomTzsrgR2StartW::new(self, 16)
    }
}
#[doc = "ROM Security Region 2 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom_tzsrg_r2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_rom_tzsrg_r2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcRomTzsrgR2Spec;
impl crate::RegisterSpec for TzcRomTzsrgR2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_rom_tzsrg_r2::R`](R) reader structure"]
impl crate::Readable for TzcRomTzsrgR2Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_rom_tzsrg_r2::W`](W) writer structure"]
impl crate::Writable for TzcRomTzsrgR2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_ROM_TZSRG_R2 to value 0x03ff"]
impl crate::Resettable for TzcRomTzsrgR2Spec {
    const RESET_VALUE: u32 = 0x03ff;
}
