#[doc = "Register `CPU_PLL_CFG2` reader"]
pub type R = crate::R<CpuPllCfg2Spec>;
#[doc = "Register `CPU_PLL_CFG2` writer"]
pub type W = crate::W<CpuPllCfg2Spec>;
#[doc = "Field `cpupll_sel_cp_bias` reader - "]
pub type CpupllSelCpBiasR = crate::BitReader;
#[doc = "Field `cpupll_sel_cp_bias` writer - "]
pub type CpupllSelCpBiasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_icp_5u` reader - "]
pub type CpupllIcp5uR = crate::FieldReader;
#[doc = "Field `cpupll_icp_5u` writer - "]
pub type CpupllIcp5uW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cpupll_icp_1u` reader - "]
pub type CpupllIcp1uR = crate::FieldReader;
#[doc = "Field `cpupll_icp_1u` writer - "]
pub type CpupllIcp1uW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cpupll_int_frac_sw` reader - "]
pub type CpupllIntFracSwR = crate::BitReader;
#[doc = "Field `cpupll_int_frac_sw` writer - "]
pub type CpupllIntFracSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_cp_startup_en` reader - "]
pub type CpupllCpStartupEnR = crate::BitReader;
#[doc = "Field `cpupll_cp_startup_en` writer - "]
pub type CpupllCpStartupEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_cp_opamp_en` reader - "]
pub type CpupllCpOpampEnR = crate::BitReader;
#[doc = "Field `cpupll_cp_opamp_en` writer - "]
pub type CpupllCpOpampEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpupll_sel_cp_bias(&self) -> CpupllSelCpBiasR {
        CpupllSelCpBiasR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn cpupll_icp_5u(&self) -> CpupllIcp5uR {
        CpupllIcp5uR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn cpupll_icp_1u(&self) -> CpupllIcp1uR {
        CpupllIcp1uR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cpupll_int_frac_sw(&self) -> CpupllIntFracSwR {
        CpupllIntFracSwR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cpupll_cp_startup_en(&self) -> CpupllCpStartupEnR {
        CpupllCpStartupEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cpupll_cp_opamp_en(&self) -> CpupllCpOpampEnR {
        CpupllCpOpampEnR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpupll_sel_cp_bias(&mut self) -> CpupllSelCpBiasW<'_, CpuPllCfg2Spec> {
        CpupllSelCpBiasW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn cpupll_icp_5u(&mut self) -> CpupllIcp5uW<'_, CpuPllCfg2Spec> {
        CpupllIcp5uW::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn cpupll_icp_1u(&mut self) -> CpupllIcp1uW<'_, CpuPllCfg2Spec> {
        CpupllIcp1uW::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cpupll_int_frac_sw(&mut self) -> CpupllIntFracSwW<'_, CpuPllCfg2Spec> {
        CpupllIntFracSwW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cpupll_cp_startup_en(&mut self) -> CpupllCpStartupEnW<'_, CpuPllCfg2Spec> {
        CpupllCpStartupEnW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cpupll_cp_opamp_en(&mut self) -> CpupllCpOpampEnW<'_, CpuPllCfg2Spec> {
        CpupllCpOpampEnW::new(self, 10)
    }
}
#[doc = "CPU PLL Configuration 2 - Charge Pump\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pll_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_pll_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuPllCfg2Spec;
impl crate::RegisterSpec for CpuPllCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_pll_cfg2::R`](R) reader structure"]
impl crate::Readable for CpuPllCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_pll_cfg2::W`](W) writer structure"]
impl crate::Writable for CpuPllCfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_PLL_CFG2 to value 0x0741"]
impl crate::Resettable for CpuPllCfg2Spec {
    const RESET_VALUE: u32 = 0x0741;
}
