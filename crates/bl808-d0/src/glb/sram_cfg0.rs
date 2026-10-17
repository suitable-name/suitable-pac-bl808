#[doc = "Register `SRAM_CFG0` reader"]
pub type R = crate::R<SramCfg0Spec>;
#[doc = "Register `SRAM_CFG0` writer"]
pub type W = crate::W<SramCfg0Spec>;
#[doc = "Field `cr_mcu_cache_ret` reader - "]
pub type CrMcuCacheRetR = crate::FieldReader;
#[doc = "Field `cr_mcu_cache_ret` writer - "]
pub type CrMcuCacheRetW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_mcu_hsram_ret` reader - "]
pub type CrMcuHsramRetR = crate::FieldReader;
#[doc = "Field `cr_mcu_hsram_ret` writer - "]
pub type CrMcuHsramRetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_wb_ram_ret` reader - "]
pub type CrWbRamRetR = crate::BitReader;
#[doc = "Field `cr_wb_ram_ret` writer - "]
pub type CrWbRamRetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_misc_ram_ret` reader - "]
pub type CrMiscRamRetR = crate::FieldReader;
#[doc = "Field `cr_misc_ram_ret` writer - "]
pub type CrMiscRamRetW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cr_mcu_cache_ret(&self) -> CrMcuCacheRetR {
        CrMcuCacheRetR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn cr_mcu_hsram_ret(&self) -> CrMcuHsramRetR {
        CrMcuHsramRetR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_wb_ram_ret(&self) -> CrWbRamRetR {
        CrWbRamRetR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn cr_misc_ram_ret(&self) -> CrMiscRamRetR {
        CrMiscRamRetR::new(((self.bits >> 9) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cr_mcu_cache_ret(&mut self) -> CrMcuCacheRetW<'_, SramCfg0Spec> {
        CrMcuCacheRetW::new(self, 0)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn cr_mcu_hsram_ret(&mut self) -> CrMcuHsramRetW<'_, SramCfg0Spec> {
        CrMcuHsramRetW::new(self, 2)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_wb_ram_ret(&mut self) -> CrWbRamRetW<'_, SramCfg0Spec> {
        CrWbRamRetW::new(self, 8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn cr_misc_ram_ret(&mut self) -> CrMiscRamRetW<'_, SramCfg0Spec> {
        CrMiscRamRetW::new(self, 9)
    }
}
#[doc = "SRAM Configuration 0 - Retention Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramCfg0Spec;
impl crate::RegisterSpec for SramCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_cfg0::R`](R) reader structure"]
impl crate::Readable for SramCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`sram_cfg0::W`](W) writer structure"]
impl crate::Writable for SramCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM_CFG0 to value 0"]
impl crate::Resettable for SramCfg0Spec {}
