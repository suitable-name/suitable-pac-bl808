#[doc = "Register `ADC_CFG0` reader"]
pub type R = crate::R<AdcCfg0Spec>;
#[doc = "Register `ADC_CFG0` writer"]
pub type W = crate::W<AdcCfg0Spec>;
#[doc = "Field `gpadc_32m_clk_div` reader - "]
pub type Gpadc32mClkDivR = crate::FieldReader;
#[doc = "Field `gpadc_32m_clk_div` writer - "]
pub type Gpadc32mClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `gpadc_32m_clk_sel` reader - "]
pub type Gpadc32mClkSelR = crate::BitReader;
#[doc = "Field `gpadc_32m_clk_sel` writer - "]
pub type Gpadc32mClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_32m_div_en` reader - "]
pub type Gpadc32mDivEnR = crate::BitReader;
#[doc = "Field `gpadc_32m_div_en` writer - "]
pub type Gpadc32mDivEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn gpadc_32m_clk_div(&self) -> Gpadc32mClkDivR {
        Gpadc32mClkDivR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpadc_32m_clk_sel(&self) -> Gpadc32mClkSelR {
        Gpadc32mClkSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_32m_div_en(&self) -> Gpadc32mDivEnR {
        Gpadc32mDivEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn gpadc_32m_clk_div(&mut self) -> Gpadc32mClkDivW<'_, AdcCfg0Spec> {
        Gpadc32mClkDivW::new(self, 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpadc_32m_clk_sel(&mut self) -> Gpadc32mClkSelW<'_, AdcCfg0Spec> {
        Gpadc32mClkSelW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_32m_div_en(&mut self) -> Gpadc32mDivEnW<'_, AdcCfg0Spec> {
        Gpadc32mDivEnW::new(self, 8)
    }
}
#[doc = "GPADC Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcCfg0Spec;
impl crate::RegisterSpec for AdcCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_cfg0::R`](R) reader structure"]
impl crate::Readable for AdcCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`adc_cfg0::W`](W) writer structure"]
impl crate::Writable for AdcCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC_CFG0 to value 0"]
impl crate::Resettable for AdcCfg0Spec {}
