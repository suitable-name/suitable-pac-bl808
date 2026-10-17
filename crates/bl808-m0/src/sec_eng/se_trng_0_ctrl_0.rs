#[doc = "Register `SE_TRNG_0_CTRL_0` reader"]
pub type R = crate::R<SeTrng0Ctrl0Spec>;
#[doc = "Register `SE_TRNG_0_CTRL_0` writer"]
pub type W = crate::W<SeTrng0Ctrl0Spec>;
#[doc = "Field `se_trng_0_busy` reader - "]
pub type SeTrng0BusyR = crate::BitReader;
#[doc = "Field `se_trng_0_trig_1t` writer - "]
pub type SeTrng0Trig1tW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_trng_0_en` reader - "]
pub type SeTrng0EnR = crate::BitReader;
#[doc = "Field `se_trng_0_en` writer - "]
pub type SeTrng0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_trng_0_dout_clr_1t` writer - "]
pub type SeTrng0DoutClr1tW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_trng_0_ht_error` reader - "]
pub type SeTrng0HtErrorR = crate::BitReader;
#[doc = "Field `se_trng_0_int` reader - "]
pub type SeTrng0IntR = crate::BitReader;
#[doc = "Field `se_trng_0_int_clr_1t` writer - "]
pub type SeTrng0IntClr1tW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_trng_0_int_set_1t` writer - "]
pub type SeTrng0IntSet1tW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_trng_0_int_mask` reader - "]
pub type SeTrng0IntMaskR = crate::BitReader;
#[doc = "Field `se_trng_0_int_mask` writer - "]
pub type SeTrng0IntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_trng_0_manual_fun_sel` reader - "]
pub type SeTrng0ManualFunSelR = crate::BitReader;
#[doc = "Field `se_trng_0_manual_fun_sel` writer - "]
pub type SeTrng0ManualFunSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_trng_0_manual_reseed` reader - "]
pub type SeTrng0ManualReseedR = crate::BitReader;
#[doc = "Field `se_trng_0_manual_reseed` writer - "]
pub type SeTrng0ManualReseedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_trng_0_manual_en` reader - "]
pub type SeTrng0ManualEnR = crate::BitReader;
#[doc = "Field `se_trng_0_manual_en` writer - "]
pub type SeTrng0ManualEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_trng_0_busy(&self) -> SeTrng0BusyR {
        SeTrng0BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_trng_0_en(&self) -> SeTrng0EnR {
        SeTrng0EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn se_trng_0_ht_error(&self) -> SeTrng0HtErrorR {
        SeTrng0HtErrorR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_trng_0_int(&self) -> SeTrng0IntR {
        SeTrng0IntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_trng_0_int_mask(&self) -> SeTrng0IntMaskR {
        SeTrng0IntMaskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_trng_0_manual_fun_sel(&self) -> SeTrng0ManualFunSelR {
        SeTrng0ManualFunSelR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn se_trng_0_manual_reseed(&self) -> SeTrng0ManualReseedR {
        SeTrng0ManualReseedR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn se_trng_0_manual_en(&self) -> SeTrng0ManualEnR {
        SeTrng0ManualEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_trng_0_trig_1t(&mut self) -> SeTrng0Trig1tW<'_, SeTrng0Ctrl0Spec> {
        SeTrng0Trig1tW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_trng_0_en(&mut self) -> SeTrng0EnW<'_, SeTrng0Ctrl0Spec> {
        SeTrng0EnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_trng_0_dout_clr_1t(&mut self) -> SeTrng0DoutClr1tW<'_, SeTrng0Ctrl0Spec> {
        SeTrng0DoutClr1tW::new(self, 3)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_trng_0_int_clr_1t(&mut self) -> SeTrng0IntClr1tW<'_, SeTrng0Ctrl0Spec> {
        SeTrng0IntClr1tW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_trng_0_int_set_1t(&mut self) -> SeTrng0IntSet1tW<'_, SeTrng0Ctrl0Spec> {
        SeTrng0IntSet1tW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_trng_0_int_mask(&mut self) -> SeTrng0IntMaskW<'_, SeTrng0Ctrl0Spec> {
        SeTrng0IntMaskW::new(self, 11)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_trng_0_manual_fun_sel(&mut self) -> SeTrng0ManualFunSelW<'_, SeTrng0Ctrl0Spec> {
        SeTrng0ManualFunSelW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn se_trng_0_manual_reseed(&mut self) -> SeTrng0ManualReseedW<'_, SeTrng0Ctrl0Spec> {
        SeTrng0ManualReseedW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn se_trng_0_manual_en(&mut self) -> SeTrng0ManualEnW<'_, SeTrng0Ctrl0Spec> {
        SeTrng0ManualEnW::new(self, 15)
    }
}
#[doc = "TRNG Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_ctrl_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_trng_0_ctrl_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeTrng0Ctrl0Spec;
impl crate::RegisterSpec for SeTrng0Ctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_trng_0_ctrl_0::R`](R) reader structure"]
impl crate::Readable for SeTrng0Ctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`se_trng_0_ctrl_0::W`](W) writer structure"]
impl crate::Writable for SeTrng0Ctrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_TRNG_0_CTRL_0 to value 0"]
impl crate::Resettable for SeTrng0Ctrl0Spec {}
