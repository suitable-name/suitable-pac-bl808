#[doc = "Register `BALI_GENERAL_CFG` reader"]
pub type R = crate::R<BaliGeneralCfgSpec>;
#[doc = "Register `BALI_GENERAL_CFG` writer"]
pub type W = crate::W<BaliGeneralCfgSpec>;
#[doc = "Field `reg_imgi_unsgn` reader - Image input unsigned mode enable (1=unsigned input data)"]
pub type RegImgiUnsgnR = crate::BitReader;
#[doc = "Field `reg_imgi_unsgn` writer - Image input unsigned mode enable (1=unsigned input data)"]
pub type RegImgiUnsgnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_img_in_mode` reader - Image Input Mode selection"]
pub type RegImgInModeR = crate::FieldReader;
#[doc = "Field `reg_img_in_mode` writer - Image Input Mode selection"]
pub type RegImgInModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_ftable_idx_base` reader - Feature Table Index Base Address (Non-zero reset value 0x5)"]
pub type RegFtableIdxBaseR = crate::FieldReader;
#[doc = "Field `reg_ftable_idx_base` writer - Feature Table Index Base Address (Non-zero reset value 0x5)"]
pub type RegFtableIdxBaseW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `reg_ftable_dat_base` reader - Feature Table Data Base Address (Non-zero reset value 0x7)"]
pub type RegFtableDatBaseR = crate::FieldReader;
#[doc = "Field `reg_ftable_dat_base` writer - Feature Table Data Base Address (Non-zero reset value 0x7)"]
pub type RegFtableDatBaseW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `axi_w_idle` reader - AXI write channel idle status"]
pub type AxiWIdleR = crate::BitReader;
#[doc = "Field `axi_r_idle` reader - AXI read channel idle status"]
pub type AxiRIdleR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Image input unsigned mode enable (1=unsigned input data)"]
    #[inline(always)]
    pub fn reg_imgi_unsgn(&self) -> RegImgiUnsgnR {
        RegImgiUnsgnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Image Input Mode selection"]
    #[inline(always)]
    pub fn reg_img_in_mode(&self) -> RegImgInModeR {
        RegImgInModeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:21 - Feature Table Index Base Address (Non-zero reset value 0x5)"]
    #[inline(always)]
    pub fn reg_ftable_idx_base(&self) -> RegFtableIdxBaseR {
        RegFtableIdxBaseR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Feature Table Data Base Address (Non-zero reset value 0x7)"]
    #[inline(always)]
    pub fn reg_ftable_dat_base(&self) -> RegFtableDatBaseR {
        RegFtableDatBaseR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - AXI write channel idle status"]
    #[inline(always)]
    pub fn axi_w_idle(&self) -> AxiWIdleR {
        AxiWIdleR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AXI read channel idle status"]
    #[inline(always)]
    pub fn axi_r_idle(&self) -> AxiRIdleR {
        AxiRIdleR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Image input unsigned mode enable (1=unsigned input data)"]
    #[inline(always)]
    pub fn reg_imgi_unsgn(&mut self) -> RegImgiUnsgnW<'_, BaliGeneralCfgSpec> {
        RegImgiUnsgnW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Image Input Mode selection"]
    #[inline(always)]
    pub fn reg_img_in_mode(&mut self) -> RegImgInModeW<'_, BaliGeneralCfgSpec> {
        RegImgInModeW::new(self, 8)
    }
    #[doc = "Bits 16:21 - Feature Table Index Base Address (Non-zero reset value 0x5)"]
    #[inline(always)]
    pub fn reg_ftable_idx_base(&mut self) -> RegFtableIdxBaseW<'_, BaliGeneralCfgSpec> {
        RegFtableIdxBaseW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Feature Table Data Base Address (Non-zero reset value 0x7)"]
    #[inline(always)]
    pub fn reg_ftable_dat_base(&mut self) -> RegFtableDatBaseW<'_, BaliGeneralCfgSpec> {
        RegFtableDatBaseW::new(self, 24)
    }
}
#[doc = "BLAI General Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bali_general_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bali_general_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaliGeneralCfgSpec;
impl crate::RegisterSpec for BaliGeneralCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bali_general_cfg::R`](R) reader structure"]
impl crate::Readable for BaliGeneralCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`bali_general_cfg::W`](W) writer structure"]
impl crate::Writable for BaliGeneralCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BALI_GENERAL_CFG to value 0x0705_0000"]
impl crate::Resettable for BaliGeneralCfgSpec {
    const RESET_VALUE: u32 = 0x0705_0000;
}
