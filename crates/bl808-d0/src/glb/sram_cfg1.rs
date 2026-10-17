#[doc = "Register `SRAM_CFG1` reader"]
pub type R = crate::R<SramCfg1Spec>;
#[doc = "Register `SRAM_CFG1` writer"]
pub type W = crate::W<SramCfg1Spec>;
#[doc = "Field `cr_mcu_cache_slp` reader - "]
pub type CrMcuCacheSlpR = crate::FieldReader;
#[doc = "Field `cr_mcu_cache_slp` writer - "]
pub type CrMcuCacheSlpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_mcu_hsram_slp` reader - "]
pub type CrMcuHsramSlpR = crate::FieldReader;
#[doc = "Field `cr_mcu_hsram_slp` writer - "]
pub type CrMcuHsramSlpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_mcu_rom_slp` reader - "]
pub type CrMcuRomSlpR = crate::FieldReader;
#[doc = "Field `cr_mcu_rom_slp` writer - "]
pub type CrMcuRomSlpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_wb_ram_slp` reader - "]
pub type CrWbRamSlpR = crate::BitReader;
#[doc = "Field `cr_wb_ram_slp` writer - "]
pub type CrWbRamSlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_misc_ram_slp` reader - "]
pub type CrMiscRamSlpR = crate::FieldReader;
#[doc = "Field `cr_misc_ram_slp` writer - "]
pub type CrMiscRamSlpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cr_mcu_cache_slp(&self) -> CrMcuCacheSlpR {
        CrMcuCacheSlpR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn cr_mcu_hsram_slp(&self) -> CrMcuHsramSlpR {
        CrMcuHsramSlpR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn cr_mcu_rom_slp(&self) -> CrMcuRomSlpR {
        CrMcuRomSlpR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_wb_ram_slp(&self) -> CrWbRamSlpR {
        CrWbRamSlpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn cr_misc_ram_slp(&self) -> CrMiscRamSlpR {
        CrMiscRamSlpR::new(((self.bits >> 9) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cr_mcu_cache_slp(&mut self) -> CrMcuCacheSlpW<'_, SramCfg1Spec> {
        CrMcuCacheSlpW::new(self, 0)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn cr_mcu_hsram_slp(&mut self) -> CrMcuHsramSlpW<'_, SramCfg1Spec> {
        CrMcuHsramSlpW::new(self, 2)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn cr_mcu_rom_slp(&mut self) -> CrMcuRomSlpW<'_, SramCfg1Spec> {
        CrMcuRomSlpW::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_wb_ram_slp(&mut self) -> CrWbRamSlpW<'_, SramCfg1Spec> {
        CrWbRamSlpW::new(self, 8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn cr_misc_ram_slp(&mut self) -> CrMiscRamSlpW<'_, SramCfg1Spec> {
        CrMiscRamSlpW::new(self, 9)
    }
}
#[doc = "SRAM Configuration 1 - Sleep Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramCfg1Spec;
impl crate::RegisterSpec for SramCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_cfg1::R`](R) reader structure"]
impl crate::Readable for SramCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`sram_cfg1::W`](W) writer structure"]
impl crate::Writable for SramCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM_CFG1 to value 0"]
impl crate::Resettable for SramCfg1Spec {}
