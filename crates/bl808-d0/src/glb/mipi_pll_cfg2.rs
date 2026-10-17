#[doc = "Register `MIPI_PLL_CFG2` reader"]
pub type R = crate::R<MipiPllCfg2Spec>;
#[doc = "Register `MIPI_PLL_CFG2` writer"]
pub type W = crate::W<MipiPllCfg2Spec>;
#[doc = "Field `mipipll_sel_cp_bias` reader - "]
pub type MipipllSelCpBiasR = crate::BitReader;
#[doc = "Field `mipipll_sel_cp_bias` writer - "]
pub type MipipllSelCpBiasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mipipll_icp_5u` reader - "]
pub type MipipllIcp5uR = crate::FieldReader;
#[doc = "Field `mipipll_icp_5u` writer - "]
pub type MipipllIcp5uW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mipipll_icp_1u` reader - "]
pub type MipipllIcp1uR = crate::FieldReader;
#[doc = "Field `mipipll_icp_1u` writer - "]
pub type MipipllIcp1uW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mipipll_int_frac_sw` reader - "]
pub type MipipllIntFracSwR = crate::BitReader;
#[doc = "Field `mipipll_int_frac_sw` writer - "]
pub type MipipllIntFracSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mipipll_cp_startup_en` reader - "]
pub type MipipllCpStartupEnR = crate::BitReader;
#[doc = "Field `mipipll_cp_startup_en` writer - "]
pub type MipipllCpStartupEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mipipll_cp_opamp_en` reader - "]
pub type MipipllCpOpampEnR = crate::BitReader;
#[doc = "Field `mipipll_cp_opamp_en` writer - "]
pub type MipipllCpOpampEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mipipll_cp_ota_en` reader - "]
pub type MipipllCpOtaEnR = crate::BitReader;
#[doc = "Field `mipipll_cp_ota_en` writer - "]
pub type MipipllCpOtaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mipipll_pfd_en` reader - "]
pub type MipipllPfdEnR = crate::BitReader;
#[doc = "Field `mipipll_pfd_en` writer - "]
pub type MipipllPfdEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mipipll_sel_cp_bias(&self) -> MipipllSelCpBiasR {
        MipipllSelCpBiasR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn mipipll_icp_5u(&self) -> MipipllIcp5uR {
        MipipllIcp5uR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn mipipll_icp_1u(&self) -> MipipllIcp1uR {
        MipipllIcp1uR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn mipipll_int_frac_sw(&self) -> MipipllIntFracSwR {
        MipipllIntFracSwR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn mipipll_cp_startup_en(&self) -> MipipllCpStartupEnR {
        MipipllCpStartupEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn mipipll_cp_opamp_en(&self) -> MipipllCpOpampEnR {
        MipipllCpOpampEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn mipipll_cp_ota_en(&self) -> MipipllCpOtaEnR {
        MipipllCpOtaEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn mipipll_pfd_en(&self) -> MipipllPfdEnR {
        MipipllPfdEnR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mipipll_sel_cp_bias(&mut self) -> MipipllSelCpBiasW<'_, MipiPllCfg2Spec> {
        MipipllSelCpBiasW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn mipipll_icp_5u(&mut self) -> MipipllIcp5uW<'_, MipiPllCfg2Spec> {
        MipipllIcp5uW::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn mipipll_icp_1u(&mut self) -> MipipllIcp1uW<'_, MipiPllCfg2Spec> {
        MipipllIcp1uW::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn mipipll_int_frac_sw(&mut self) -> MipipllIntFracSwW<'_, MipiPllCfg2Spec> {
        MipipllIntFracSwW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn mipipll_cp_startup_en(&mut self) -> MipipllCpStartupEnW<'_, MipiPllCfg2Spec> {
        MipipllCpStartupEnW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn mipipll_cp_opamp_en(&mut self) -> MipipllCpOpampEnW<'_, MipiPllCfg2Spec> {
        MipipllCpOpampEnW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn mipipll_cp_ota_en(&mut self) -> MipipllCpOtaEnW<'_, MipiPllCfg2Spec> {
        MipipllCpOtaEnW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn mipipll_pfd_en(&mut self) -> MipipllPfdEnW<'_, MipiPllCfg2Spec> {
        MipipllPfdEnW::new(self, 12)
    }
}
#[doc = "MIPI PLL Configuration 2 - Charge Pump\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_pll_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_pll_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiPllCfg2Spec;
impl crate::RegisterSpec for MipiPllCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_pll_cfg2::R`](R) reader structure"]
impl crate::Readable for MipiPllCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`mipi_pll_cfg2::W`](W) writer structure"]
impl crate::Writable for MipiPllCfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIPI_PLL_CFG2 to value 0"]
impl crate::Resettable for MipiPllCfg2Spec {}
