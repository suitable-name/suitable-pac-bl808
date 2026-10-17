#[doc = "Register `TOUCH1` reader"]
pub type R = crate::R<Touch1Spec>;
#[doc = "Register `TOUCH1` writer"]
pub type W = crate::W<Touch1Spec>;
#[doc = "Field `touch_vref_sel` reader - "]
pub type TouchVrefSelR = crate::FieldReader;
#[doc = "Field `touch_vref_sel` writer - "]
pub type TouchVrefSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `touch_vldo_sel` reader - "]
pub type TouchVldoSelR = crate::FieldReader;
#[doc = "Field `touch_vldo_sel` writer - "]
pub type TouchVldoSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `touch_comp_hys_sel` reader - "]
pub type TouchCompHysSelR = crate::BitReader;
#[doc = "Field `touch_comp_hys_sel` writer - "]
pub type TouchCompHysSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `touch_current_sel` reader - "]
pub type TouchCurrentSelR = crate::BitReader;
#[doc = "Field `touch_current_sel` writer - "]
pub type TouchCurrentSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `touch_clk_sel` reader - "]
pub type TouchClkSelR = crate::BitReader;
#[doc = "Field `touch_clk_sel` writer - "]
pub type TouchClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `touch_clk_div_ratio` reader - "]
pub type TouchClkDivRatioR = crate::FieldReader;
#[doc = "Field `touch_clk_div_ratio` writer - "]
pub type TouchClkDivRatioW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `touch_pcharge_high` reader - "]
pub type TouchPchargeHighR = crate::FieldReader;
#[doc = "Field `touch_pcharge_high` writer - "]
pub type TouchPchargeHighW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `touch_pcharge_low` reader - "]
pub type TouchPchargeLowR = crate::FieldReader;
#[doc = "Field `touch_pcharge_low` writer - "]
pub type TouchPchargeLowW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `touch_cont_en` reader - "]
pub type TouchContEnR = crate::BitReader;
#[doc = "Field `touch_cont_en` writer - "]
pub type TouchContEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `touch_cycle_en` reader - "]
pub type TouchCycleEnR = crate::BitReader;
#[doc = "Field `touch_cycle_en` writer - "]
pub type TouchCycleEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `touch_ulp_en` reader - "]
pub type TouchUlpEnR = crate::BitReader;
#[doc = "Field `touch_ulp_en` writer - "]
pub type TouchUlpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_touch` reader - "]
pub type PuTouchR = crate::BitReader;
#[doc = "Field `pu_touch` writer - "]
pub type PuTouchW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn touch_vref_sel(&self) -> TouchVrefSelR {
        TouchVrefSelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn touch_vldo_sel(&self) -> TouchVldoSelR {
        TouchVldoSelR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn touch_comp_hys_sel(&self) -> TouchCompHysSelR {
        TouchCompHysSelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn touch_current_sel(&self) -> TouchCurrentSelR {
        TouchCurrentSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn touch_clk_sel(&self) -> TouchClkSelR {
        TouchClkSelR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn touch_clk_div_ratio(&self) -> TouchClkDivRatioR {
        TouchClkDivRatioR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn touch_pcharge_high(&self) -> TouchPchargeHighR {
        TouchPchargeHighR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn touch_pcharge_low(&self) -> TouchPchargeLowR {
        TouchPchargeLowR::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn touch_cont_en(&self) -> TouchContEnR {
        TouchContEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn touch_cycle_en(&self) -> TouchCycleEnR {
        TouchCycleEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn touch_ulp_en(&self) -> TouchUlpEnR {
        TouchUlpEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pu_touch(&self) -> PuTouchR {
        PuTouchR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn touch_vref_sel(&mut self) -> TouchVrefSelW<'_, Touch1Spec> {
        TouchVrefSelW::new(self, 0)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn touch_vldo_sel(&mut self) -> TouchVldoSelW<'_, Touch1Spec> {
        TouchVldoSelW::new(self, 3)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn touch_comp_hys_sel(&mut self) -> TouchCompHysSelW<'_, Touch1Spec> {
        TouchCompHysSelW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn touch_current_sel(&mut self) -> TouchCurrentSelW<'_, Touch1Spec> {
        TouchCurrentSelW::new(self, 7)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn touch_clk_sel(&mut self) -> TouchClkSelW<'_, Touch1Spec> {
        TouchClkSelW::new(self, 16)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn touch_clk_div_ratio(&mut self) -> TouchClkDivRatioW<'_, Touch1Spec> {
        TouchClkDivRatioW::new(self, 17)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn touch_pcharge_high(&mut self) -> TouchPchargeHighW<'_, Touch1Spec> {
        TouchPchargeHighW::new(self, 20)
    }
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn touch_pcharge_low(&mut self) -> TouchPchargeLowW<'_, Touch1Spec> {
        TouchPchargeLowW::new(self, 23)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn touch_cont_en(&mut self) -> TouchContEnW<'_, Touch1Spec> {
        TouchContEnW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn touch_cycle_en(&mut self) -> TouchCycleEnW<'_, Touch1Spec> {
        TouchCycleEnW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn touch_ulp_en(&mut self) -> TouchUlpEnW<'_, Touch1Spec> {
        TouchUlpEnW::new(self, 28)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pu_touch(&mut self) -> PuTouchW<'_, Touch1Spec> {
        PuTouchW::new(self, 30)
    }
}
#[doc = "Touch Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`touch1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Touch1Spec;
impl crate::RegisterSpec for Touch1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch1::R`](R) reader structure"]
impl crate::Readable for Touch1Spec {}
#[doc = "`write(|w| ..)` method takes [`touch1::W`](W) writer structure"]
impl crate::Writable for Touch1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH1 to value 0x00a3_001b"]
impl crate::Resettable for Touch1Spec {
    const RESET_VALUE: u32 = 0x00a3_001b;
}
