#[doc = "Register `I2S_CFG0` reader"]
pub type R = crate::R<I2sCfg0Spec>;
#[doc = "Register `I2S_CFG0` writer"]
pub type W = crate::W<I2sCfg0Spec>;
#[doc = "Field `reg_i2s_ref_clk_div` reader - "]
pub type RegI2sRefClkDivR = crate::FieldReader;
#[doc = "Field `reg_i2s_ref_clk_div` writer - "]
pub type RegI2sRefClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `reg_i2s_di_ref_clk_sel` reader - "]
pub type RegI2sDiRefClkSelR = crate::BitReader;
#[doc = "Field `reg_i2s_di_ref_clk_sel` writer - "]
pub type RegI2sDiRefClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_i2s_ref_clk_en` reader - "]
pub type RegI2sRefClkEnR = crate::BitReader;
#[doc = "Field `reg_i2s_ref_clk_en` writer - "]
pub type RegI2sRefClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_i2s_do_ref_clk_sel` reader - "]
pub type RegI2sDoRefClkSelR = crate::BitReader;
#[doc = "Field `reg_i2s_do_ref_clk_sel` writer - "]
pub type RegI2sDoRefClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn reg_i2s_ref_clk_div(&self) -> RegI2sRefClkDivR {
        RegI2sRefClkDivR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_i2s_di_ref_clk_sel(&self) -> RegI2sDiRefClkSelR {
        RegI2sDiRefClkSelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_i2s_ref_clk_en(&self) -> RegI2sRefClkEnR {
        RegI2sRefClkEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_i2s_do_ref_clk_sel(&self) -> RegI2sDoRefClkSelR {
        RegI2sDoRefClkSelR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn reg_i2s_ref_clk_div(&mut self) -> RegI2sRefClkDivW<'_, I2sCfg0Spec> {
        RegI2sRefClkDivW::new(self, 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_i2s_di_ref_clk_sel(&mut self) -> RegI2sDiRefClkSelW<'_, I2sCfg0Spec> {
        RegI2sDiRefClkSelW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_i2s_ref_clk_en(&mut self) -> RegI2sRefClkEnW<'_, I2sCfg0Spec> {
        RegI2sRefClkEnW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_i2s_do_ref_clk_sel(&mut self) -> RegI2sDoRefClkSelW<'_, I2sCfg0Spec> {
        RegI2sDoRefClkSelW::new(self, 8)
    }
}
#[doc = "I2S Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sCfg0Spec;
impl crate::RegisterSpec for I2sCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_cfg0::R`](R) reader structure"]
impl crate::Readable for I2sCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`i2s_cfg0::W`](W) writer structure"]
impl crate::Writable for I2sCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2S_CFG0 to value 0"]
impl crate::Resettable for I2sCfg0Spec {}
