#[doc = "Register `UHS_PLL_CFG2` reader"]
pub type R = crate::R<UhsPllCfg2Spec>;
#[doc = "Register `UHS_PLL_CFG2` writer"]
pub type W = crate::W<UhsPllCfg2Spec>;
#[doc = "Field `uhspll_sel_cp_bias` reader - "]
pub type UhspllSelCpBiasR = crate::BitReader;
#[doc = "Field `uhspll_sel_cp_bias` writer - "]
pub type UhspllSelCpBiasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhspll_icp_5u` reader - "]
pub type UhspllIcp5uR = crate::FieldReader;
#[doc = "Field `uhspll_icp_5u` writer - "]
pub type UhspllIcp5uW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `uhspll_icp_1u` reader - "]
pub type UhspllIcp1uR = crate::FieldReader;
#[doc = "Field `uhspll_icp_1u` writer - "]
pub type UhspllIcp1uW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `uhspll_int_frac_sw` reader - "]
pub type UhspllIntFracSwR = crate::BitReader;
#[doc = "Field `uhspll_int_frac_sw` writer - "]
pub type UhspllIntFracSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhspll_cp_startup_en` reader - "]
pub type UhspllCpStartupEnR = crate::BitReader;
#[doc = "Field `uhspll_cp_startup_en` writer - "]
pub type UhspllCpStartupEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhspll_cp_opamp_en` reader - "]
pub type UhspllCpOpampEnR = crate::BitReader;
#[doc = "Field `uhspll_cp_opamp_en` writer - "]
pub type UhspllCpOpampEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhspll_cp_ota_en` reader - "]
pub type UhspllCpOtaEnR = crate::BitReader;
#[doc = "Field `uhspll_cp_ota_en` writer - "]
pub type UhspllCpOtaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhspll_pfd_en` reader - "]
pub type UhspllPfdEnR = crate::BitReader;
#[doc = "Field `uhspll_pfd_en` writer - "]
pub type UhspllPfdEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhspll_sel_cp_bias(&self) -> UhspllSelCpBiasR {
        UhspllSelCpBiasR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn uhspll_icp_5u(&self) -> UhspllIcp5uR {
        UhspllIcp5uR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn uhspll_icp_1u(&self) -> UhspllIcp1uR {
        UhspllIcp1uR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhspll_int_frac_sw(&self) -> UhspllIntFracSwR {
        UhspllIntFracSwR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn uhspll_cp_startup_en(&self) -> UhspllCpStartupEnR {
        UhspllCpStartupEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn uhspll_cp_opamp_en(&self) -> UhspllCpOpampEnR {
        UhspllCpOpampEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn uhspll_cp_ota_en(&self) -> UhspllCpOtaEnR {
        UhspllCpOtaEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uhspll_pfd_en(&self) -> UhspllPfdEnR {
        UhspllPfdEnR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhspll_sel_cp_bias(&mut self) -> UhspllSelCpBiasW<'_, UhsPllCfg2Spec> {
        UhspllSelCpBiasW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn uhspll_icp_5u(&mut self) -> UhspllIcp5uW<'_, UhsPllCfg2Spec> {
        UhspllIcp5uW::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn uhspll_icp_1u(&mut self) -> UhspllIcp1uW<'_, UhsPllCfg2Spec> {
        UhspllIcp1uW::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhspll_int_frac_sw(&mut self) -> UhspllIntFracSwW<'_, UhsPllCfg2Spec> {
        UhspllIntFracSwW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn uhspll_cp_startup_en(&mut self) -> UhspllCpStartupEnW<'_, UhsPllCfg2Spec> {
        UhspllCpStartupEnW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn uhspll_cp_opamp_en(&mut self) -> UhspllCpOpampEnW<'_, UhsPllCfg2Spec> {
        UhspllCpOpampEnW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn uhspll_cp_ota_en(&mut self) -> UhspllCpOtaEnW<'_, UhsPllCfg2Spec> {
        UhspllCpOtaEnW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uhspll_pfd_en(&mut self) -> UhspllPfdEnW<'_, UhsPllCfg2Spec> {
        UhspllPfdEnW::new(self, 12)
    }
}
#[doc = "UHS PLL Configuration 2 - Charge Pump\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_pll_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_pll_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsPllCfg2Spec;
impl crate::RegisterSpec for UhsPllCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_pll_cfg2::R`](R) reader structure"]
impl crate::Readable for UhsPllCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`uhs_pll_cfg2::W`](W) writer structure"]
impl crate::Writable for UhsPllCfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHS_PLL_CFG2 to value 0"]
impl crate::Resettable for UhsPllCfg2Spec {}
