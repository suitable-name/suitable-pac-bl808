#[doc = "Register `CPU_PLL_CFG3` reader"]
pub type R = crate::R<CpuPllCfg3Spec>;
#[doc = "Register `CPU_PLL_CFG3` writer"]
pub type W = crate::W<CpuPllCfg3Spec>;
#[doc = "Field `cpupll_c4_en` reader - "]
pub type CpupllC4EnR = crate::BitReader;
#[doc = "Field `cpupll_c4_en` writer - "]
pub type CpupllC4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_r4` reader - "]
pub type CpupllR4R = crate::FieldReader;
#[doc = "Field `cpupll_r4` writer - "]
pub type CpupllR4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cpupll_r4_short` reader - "]
pub type CpupllR4ShortR = crate::BitReader;
#[doc = "Field `cpupll_r4_short` writer - "]
pub type CpupllR4ShortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_c3` reader - "]
pub type CpupllC3R = crate::FieldReader;
#[doc = "Field `cpupll_c3` writer - "]
pub type CpupllC3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cpupll_cz` reader - "]
pub type CpupllCzR = crate::FieldReader;
#[doc = "Field `cpupll_cz` writer - "]
pub type CpupllCzW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cpupll_rz` reader - "]
pub type CpupllRzR = crate::FieldReader;
#[doc = "Field `cpupll_rz` writer - "]
pub type CpupllRzW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpupll_c4_en(&self) -> CpupllC4EnR {
        CpupllC4EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn cpupll_r4(&self) -> CpupllR4R {
        CpupllR4R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cpupll_r4_short(&self) -> CpupllR4ShortR {
        CpupllR4ShortR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn cpupll_c3(&self) -> CpupllC3R {
        CpupllC3R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn cpupll_cz(&self) -> CpupllCzR {
        CpupllCzR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn cpupll_rz(&self) -> CpupllRzR {
        CpupllRzR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpupll_c4_en(&mut self) -> CpupllC4EnW<'_, CpuPllCfg3Spec> {
        CpupllC4EnW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn cpupll_r4(&mut self) -> CpupllR4W<'_, CpuPllCfg3Spec> {
        CpupllR4W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cpupll_r4_short(&mut self) -> CpupllR4ShortW<'_, CpuPllCfg3Spec> {
        CpupllR4ShortW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn cpupll_c3(&mut self) -> CpupllC3W<'_, CpuPllCfg3Spec> {
        CpupllC3W::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn cpupll_cz(&mut self) -> CpupllCzW<'_, CpuPllCfg3Spec> {
        CpupllCzW::new(self, 14)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn cpupll_rz(&mut self) -> CpupllRzW<'_, CpuPllCfg3Spec> {
        CpupllRzW::new(self, 16)
    }
}
#[doc = "CPU PLL Configuration 3 - Loop Filter\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pll_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_pll_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuPllCfg3Spec;
impl crate::RegisterSpec for CpuPllCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_pll_cfg3::R`](R) reader structure"]
impl crate::Readable for CpuPllCfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_pll_cfg3::W`](W) writer structure"]
impl crate::Writable for CpuPllCfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_PLL_CFG3 to value 0x0005_a021"]
impl crate::Resettable for CpuPllCfg3Spec {
    const RESET_VALUE: u32 = 0x0005_a021;
}
