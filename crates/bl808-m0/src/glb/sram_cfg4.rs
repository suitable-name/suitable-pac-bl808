#[doc = "Register `SRAM_CFG4` reader"]
pub type R = crate::R<SramCfg4Spec>;
#[doc = "Register `SRAM_CFG4` writer"]
pub type W = crate::W<SramCfg4Spec>;
#[doc = "Field `cr_mcu_cache_dvs` reader - "]
pub type CrMcuCacheDvsR = crate::FieldReader;
#[doc = "Field `cr_mcu_cache_dvs` writer - "]
pub type CrMcuCacheDvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_mcu_hsram_dvs` reader - "]
pub type CrMcuHsramDvsR = crate::FieldReader;
#[doc = "Field `cr_mcu_hsram_dvs` writer - "]
pub type CrMcuHsramDvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_mcu_rom_dvs` reader - "]
pub type CrMcuRomDvsR = crate::FieldReader;
#[doc = "Field `cr_mcu_rom_dvs` writer - "]
pub type CrMcuRomDvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_wb_ram_dvs` reader - "]
pub type CrWbRamDvsR = crate::FieldReader;
#[doc = "Field `cr_wb_ram_dvs` writer - "]
pub type CrWbRamDvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_misc_ram_dvs` reader - "]
pub type CrMiscRamDvsR = crate::FieldReader;
#[doc = "Field `cr_misc_ram_dvs` writer - "]
pub type CrMiscRamDvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_ocram_dvs` reader - "]
pub type CrOcramDvsR = crate::FieldReader;
#[doc = "Field `cr_ocram_dvs` writer - "]
pub type CrOcramDvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_wram_dvs` reader - "]
pub type CrWramDvsR = crate::FieldReader;
#[doc = "Field `cr_wram_dvs` writer - "]
pub type CrWramDvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cr_mcu_cache_dvs(&self) -> CrMcuCacheDvsR {
        CrMcuCacheDvsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cr_mcu_hsram_dvs(&self) -> CrMcuHsramDvsR {
        CrMcuHsramDvsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn cr_mcu_rom_dvs(&self) -> CrMcuRomDvsR {
        CrMcuRomDvsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cr_wb_ram_dvs(&self) -> CrWbRamDvsR {
        CrWbRamDvsR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn cr_misc_ram_dvs(&self) -> CrMiscRamDvsR {
        CrMiscRamDvsR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn cr_ocram_dvs(&self) -> CrOcramDvsR {
        CrOcramDvsR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn cr_wram_dvs(&self) -> CrWramDvsR {
        CrWramDvsR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cr_mcu_cache_dvs(&mut self) -> CrMcuCacheDvsW<'_, SramCfg4Spec> {
        CrMcuCacheDvsW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cr_mcu_hsram_dvs(&mut self) -> CrMcuHsramDvsW<'_, SramCfg4Spec> {
        CrMcuHsramDvsW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn cr_mcu_rom_dvs(&mut self) -> CrMcuRomDvsW<'_, SramCfg4Spec> {
        CrMcuRomDvsW::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cr_wb_ram_dvs(&mut self) -> CrWbRamDvsW<'_, SramCfg4Spec> {
        CrWbRamDvsW::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn cr_misc_ram_dvs(&mut self) -> CrMiscRamDvsW<'_, SramCfg4Spec> {
        CrMiscRamDvsW::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn cr_ocram_dvs(&mut self) -> CrOcramDvsW<'_, SramCfg4Spec> {
        CrOcramDvsW::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn cr_wram_dvs(&mut self) -> CrWramDvsW<'_, SramCfg4Spec> {
        CrWramDvsW::new(self, 24)
    }
}
#[doc = "SRAM Configuration 4 - DVS Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_cfg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_cfg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramCfg4Spec;
impl crate::RegisterSpec for SramCfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_cfg4::R`](R) reader structure"]
impl crate::Readable for SramCfg4Spec {}
#[doc = "`write(|w| ..)` method takes [`sram_cfg4::W`](W) writer structure"]
impl crate::Writable for SramCfg4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM_CFG4 to value 0"]
impl crate::Resettable for SramCfg4Spec {}
