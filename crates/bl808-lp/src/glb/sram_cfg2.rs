#[doc = "Register `SRAM_CFG2` reader"]
pub type R = crate::R<SramCfg2Spec>;
#[doc = "Register `SRAM_CFG2` writer"]
pub type W = crate::W<SramCfg2Spec>;
#[doc = "Field `cr_mcu_cache_dvse` reader - "]
pub type CrMcuCacheDvseR = crate::BitReader;
#[doc = "Field `cr_mcu_cache_dvse` writer - "]
pub type CrMcuCacheDvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_mcu_hsram_dvse` reader - "]
pub type CrMcuHsramDvseR = crate::BitReader;
#[doc = "Field `cr_mcu_hsram_dvse` writer - "]
pub type CrMcuHsramDvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_mcu_rom_dvse` reader - "]
pub type CrMcuRomDvseR = crate::BitReader;
#[doc = "Field `cr_mcu_rom_dvse` writer - "]
pub type CrMcuRomDvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_wb_ram_dvse` reader - "]
pub type CrWbRamDvseR = crate::BitReader;
#[doc = "Field `cr_wb_ram_dvse` writer - "]
pub type CrWbRamDvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_misc_ram_dvse` reader - "]
pub type CrMiscRamDvseR = crate::BitReader;
#[doc = "Field `cr_misc_ram_dvse` writer - "]
pub type CrMiscRamDvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_ocram_dvse` reader - "]
pub type CrOcramDvseR = crate::BitReader;
#[doc = "Field `cr_ocram_dvse` writer - "]
pub type CrOcramDvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_wram_dvse` reader - "]
pub type CrWramDvseR = crate::BitReader;
#[doc = "Field `cr_wram_dvse` writer - "]
pub type CrWramDvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_mcu_cache_nap` reader - "]
pub type CrMcuCacheNapR = crate::BitReader;
#[doc = "Field `cr_mcu_cache_nap` writer - "]
pub type CrMcuCacheNapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_mcu_hsram_nap` reader - "]
pub type CrMcuHsramNapR = crate::BitReader;
#[doc = "Field `cr_mcu_hsram_nap` writer - "]
pub type CrMcuHsramNapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_wb_ram_nap` reader - "]
pub type CrWbRamNapR = crate::BitReader;
#[doc = "Field `cr_wb_ram_nap` writer - "]
pub type CrWbRamNapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_misc_ram_nap` reader - "]
pub type CrMiscRamNapR = crate::BitReader;
#[doc = "Field `cr_misc_ram_nap` writer - "]
pub type CrMiscRamNapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_ocram_nap` reader - "]
pub type CrOcramNapR = crate::BitReader;
#[doc = "Field `cr_ocram_nap` writer - "]
pub type CrOcramNapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_wram_nap` reader - "]
pub type CrWramNapR = crate::BitReader;
#[doc = "Field `cr_wram_nap` writer - "]
pub type CrWramNapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_mcu_cache_dvse(&self) -> CrMcuCacheDvseR {
        CrMcuCacheDvseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_mcu_hsram_dvse(&self) -> CrMcuHsramDvseR {
        CrMcuHsramDvseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_mcu_rom_dvse(&self) -> CrMcuRomDvseR {
        CrMcuRomDvseR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_wb_ram_dvse(&self) -> CrWbRamDvseR {
        CrWbRamDvseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_misc_ram_dvse(&self) -> CrMiscRamDvseR {
        CrMiscRamDvseR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_ocram_dvse(&self) -> CrOcramDvseR {
        CrOcramDvseR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_wram_dvse(&self) -> CrWramDvseR {
        CrWramDvseR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_mcu_cache_nap(&self) -> CrMcuCacheNapR {
        CrMcuCacheNapR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_mcu_hsram_nap(&self) -> CrMcuHsramNapR {
        CrMcuHsramNapR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_wb_ram_nap(&self) -> CrWbRamNapR {
        CrWbRamNapR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_misc_ram_nap(&self) -> CrMiscRamNapR {
        CrMiscRamNapR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_ocram_nap(&self) -> CrOcramNapR {
        CrOcramNapR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_wram_nap(&self) -> CrWramNapR {
        CrWramNapR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_mcu_cache_dvse(&mut self) -> CrMcuCacheDvseW<'_, SramCfg2Spec> {
        CrMcuCacheDvseW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_mcu_hsram_dvse(&mut self) -> CrMcuHsramDvseW<'_, SramCfg2Spec> {
        CrMcuHsramDvseW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_mcu_rom_dvse(&mut self) -> CrMcuRomDvseW<'_, SramCfg2Spec> {
        CrMcuRomDvseW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_wb_ram_dvse(&mut self) -> CrWbRamDvseW<'_, SramCfg2Spec> {
        CrWbRamDvseW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_misc_ram_dvse(&mut self) -> CrMiscRamDvseW<'_, SramCfg2Spec> {
        CrMiscRamDvseW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_ocram_dvse(&mut self) -> CrOcramDvseW<'_, SramCfg2Spec> {
        CrOcramDvseW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_wram_dvse(&mut self) -> CrWramDvseW<'_, SramCfg2Spec> {
        CrWramDvseW::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_mcu_cache_nap(&mut self) -> CrMcuCacheNapW<'_, SramCfg2Spec> {
        CrMcuCacheNapW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_mcu_hsram_nap(&mut self) -> CrMcuHsramNapW<'_, SramCfg2Spec> {
        CrMcuHsramNapW::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_wb_ram_nap(&mut self) -> CrWbRamNapW<'_, SramCfg2Spec> {
        CrWbRamNapW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_misc_ram_nap(&mut self) -> CrMiscRamNapW<'_, SramCfg2Spec> {
        CrMiscRamNapW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_ocram_nap(&mut self) -> CrOcramNapW<'_, SramCfg2Spec> {
        CrOcramNapW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_wram_nap(&mut self) -> CrWramNapW<'_, SramCfg2Spec> {
        CrWramNapW::new(self, 14)
    }
}
#[doc = "SRAM Configuration 2 - DVSE and NAP Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramCfg2Spec;
impl crate::RegisterSpec for SramCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_cfg2::R`](R) reader structure"]
impl crate::Readable for SramCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`sram_cfg2::W`](W) writer structure"]
impl crate::Writable for SramCfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM_CFG2 to value 0"]
impl crate::Resettable for SramCfg2Spec {}
