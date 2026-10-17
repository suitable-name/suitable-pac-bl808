#[doc = "Register `MIPI_PLL_CFG3` reader"]
pub type R = crate::R<MipiPllCfg3Spec>;
#[doc = "Register `MIPI_PLL_CFG3` writer"]
pub type W = crate::W<MipiPllCfg3Spec>;
#[doc = "Field `mipipll_c4_en` reader - "]
pub type MipipllC4EnR = crate::BitReader;
#[doc = "Field `mipipll_c4_en` writer - "]
pub type MipipllC4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mipipll_r4` reader - "]
pub type MipipllR4R = crate::FieldReader;
#[doc = "Field `mipipll_r4` writer - "]
pub type MipipllR4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mipipll_r4_short` reader - "]
pub type MipipllR4ShortR = crate::BitReader;
#[doc = "Field `mipipll_r4_short` writer - "]
pub type MipipllR4ShortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mipipll_c3` reader - "]
pub type MipipllC3R = crate::FieldReader;
#[doc = "Field `mipipll_c3` writer - "]
pub type MipipllC3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mipipll_cz` reader - "]
pub type MipipllCzR = crate::FieldReader;
#[doc = "Field `mipipll_cz` writer - "]
pub type MipipllCzW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mipipll_rz` reader - "]
pub type MipipllRzR = crate::FieldReader;
#[doc = "Field `mipipll_rz` writer - "]
pub type MipipllRzW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `mipipll_lf_test_en` reader - "]
pub type MipipllLfTestEnR = crate::BitReader;
#[doc = "Field `mipipll_lf_test_en` writer - "]
pub type MipipllLfTestEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mipipll_fast_lock_en` reader - "]
pub type MipipllFastLockEnR = crate::BitReader;
#[doc = "Field `mipipll_fast_lock_en` writer - "]
pub type MipipllFastLockEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mipipll_c4_en(&self) -> MipipllC4EnR {
        MipipllC4EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn mipipll_r4(&self) -> MipipllR4R {
        MipipllR4R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn mipipll_r4_short(&self) -> MipipllR4ShortR {
        MipipllR4ShortR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn mipipll_c3(&self) -> MipipllC3R {
        MipipllC3R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn mipipll_cz(&self) -> MipipllCzR {
        MipipllCzR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn mipipll_rz(&self) -> MipipllRzR {
        MipipllRzR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn mipipll_lf_test_en(&self) -> MipipllLfTestEnR {
        MipipllLfTestEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn mipipll_fast_lock_en(&self) -> MipipllFastLockEnR {
        MipipllFastLockEnR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mipipll_c4_en(&mut self) -> MipipllC4EnW<'_, MipiPllCfg3Spec> {
        MipipllC4EnW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn mipipll_r4(&mut self) -> MipipllR4W<'_, MipiPllCfg3Spec> {
        MipipllR4W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn mipipll_r4_short(&mut self) -> MipipllR4ShortW<'_, MipiPllCfg3Spec> {
        MipipllR4ShortW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn mipipll_c3(&mut self) -> MipipllC3W<'_, MipiPllCfg3Spec> {
        MipipllC3W::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn mipipll_cz(&mut self) -> MipipllCzW<'_, MipiPllCfg3Spec> {
        MipipllCzW::new(self, 14)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn mipipll_rz(&mut self) -> MipipllRzW<'_, MipiPllCfg3Spec> {
        MipipllRzW::new(self, 16)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn mipipll_lf_test_en(&mut self) -> MipipllLfTestEnW<'_, MipiPllCfg3Spec> {
        MipipllLfTestEnW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn mipipll_fast_lock_en(&mut self) -> MipipllFastLockEnW<'_, MipiPllCfg3Spec> {
        MipipllFastLockEnW::new(self, 20)
    }
}
#[doc = "MIPI PLL Configuration 3 - Loop Filter\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_pll_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_pll_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiPllCfg3Spec;
impl crate::RegisterSpec for MipiPllCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_pll_cfg3::R`](R) reader structure"]
impl crate::Readable for MipiPllCfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`mipi_pll_cfg3::W`](W) writer structure"]
impl crate::Writable for MipiPllCfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIPI_PLL_CFG3 to value 0"]
impl crate::Resettable for MipiPllCfg3Spec {}
