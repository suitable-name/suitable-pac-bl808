#[doc = "Register `UHS_PLL_CFG3` reader"]
pub type R = crate::R<UhsPllCfg3Spec>;
#[doc = "Register `UHS_PLL_CFG3` writer"]
pub type W = crate::W<UhsPllCfg3Spec>;
#[doc = "Field `uhspll_c4_en` reader - "]
pub type UhspllC4EnR = crate::BitReader;
#[doc = "Field `uhspll_c4_en` writer - "]
pub type UhspllC4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhspll_r4` reader - "]
pub type UhspllR4R = crate::FieldReader;
#[doc = "Field `uhspll_r4` writer - "]
pub type UhspllR4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `uhspll_r4_short` reader - "]
pub type UhspllR4ShortR = crate::BitReader;
#[doc = "Field `uhspll_r4_short` writer - "]
pub type UhspllR4ShortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhspll_c3` reader - "]
pub type UhspllC3R = crate::FieldReader;
#[doc = "Field `uhspll_c3` writer - "]
pub type UhspllC3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `uhspll_cz` reader - "]
pub type UhspllCzR = crate::FieldReader;
#[doc = "Field `uhspll_cz` writer - "]
pub type UhspllCzW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `uhspll_rz` reader - "]
pub type UhspllRzR = crate::FieldReader;
#[doc = "Field `uhspll_rz` writer - "]
pub type UhspllRzW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `uhspll_lf_test_en` reader - "]
pub type UhspllLfTestEnR = crate::BitReader;
#[doc = "Field `uhspll_lf_test_en` writer - "]
pub type UhspllLfTestEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhspll_fast_lock_en` reader - "]
pub type UhspllFastLockEnR = crate::BitReader;
#[doc = "Field `uhspll_fast_lock_en` writer - "]
pub type UhspllFastLockEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhspll_c4_en(&self) -> UhspllC4EnR {
        UhspllC4EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn uhspll_r4(&self) -> UhspllR4R {
        UhspllR4R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhspll_r4_short(&self) -> UhspllR4ShortR {
        UhspllR4ShortR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn uhspll_c3(&self) -> UhspllC3R {
        UhspllC3R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn uhspll_cz(&self) -> UhspllCzR {
        UhspllCzR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn uhspll_rz(&self) -> UhspllRzR {
        UhspllRzR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn uhspll_lf_test_en(&self) -> UhspllLfTestEnR {
        UhspllLfTestEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn uhspll_fast_lock_en(&self) -> UhspllFastLockEnR {
        UhspllFastLockEnR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhspll_c4_en(&mut self) -> UhspllC4EnW<'_, UhsPllCfg3Spec> {
        UhspllC4EnW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn uhspll_r4(&mut self) -> UhspllR4W<'_, UhsPllCfg3Spec> {
        UhspllR4W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhspll_r4_short(&mut self) -> UhspllR4ShortW<'_, UhsPllCfg3Spec> {
        UhspllR4ShortW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn uhspll_c3(&mut self) -> UhspllC3W<'_, UhsPllCfg3Spec> {
        UhspllC3W::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn uhspll_cz(&mut self) -> UhspllCzW<'_, UhsPllCfg3Spec> {
        UhspllCzW::new(self, 14)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn uhspll_rz(&mut self) -> UhspllRzW<'_, UhsPllCfg3Spec> {
        UhspllRzW::new(self, 16)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn uhspll_lf_test_en(&mut self) -> UhspllLfTestEnW<'_, UhsPllCfg3Spec> {
        UhspllLfTestEnW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn uhspll_fast_lock_en(&mut self) -> UhspllFastLockEnW<'_, UhsPllCfg3Spec> {
        UhspllFastLockEnW::new(self, 20)
    }
}
#[doc = "UHS PLL Configuration 3 - Loop Filter\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_pll_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_pll_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsPllCfg3Spec;
impl crate::RegisterSpec for UhsPllCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_pll_cfg3::R`](R) reader structure"]
impl crate::Readable for UhsPllCfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`uhs_pll_cfg3::W`](W) writer structure"]
impl crate::Writable for UhsPllCfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHS_PLL_CFG3 to value 0"]
impl crate::Resettable for UhsPllCfg3Spec {}
