#[doc = "Register `I2C_CFG0` reader"]
pub type R = crate::R<I2cCfg0Spec>;
#[doc = "Register `I2C_CFG0` writer"]
pub type W = crate::W<I2cCfg0Spec>;
#[doc = "Field `i2c_clk_div` reader - "]
pub type I2cClkDivR = crate::FieldReader;
#[doc = "Field `i2c_clk_div` writer - "]
pub type I2cClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `i2c_clk_en` reader - "]
pub type I2cClkEnR = crate::BitReader;
#[doc = "Field `i2c_clk_en` writer - "]
pub type I2cClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c_clk_sel` reader - "]
pub type I2cClkSelR = crate::BitReader;
#[doc = "Field `i2c_clk_sel` writer - "]
pub type I2cClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn i2c_clk_div(&self) -> I2cClkDivR {
        I2cClkDivR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn i2c_clk_en(&self) -> I2cClkEnR {
        I2cClkEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn i2c_clk_sel(&self) -> I2cClkSelR {
        I2cClkSelR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn i2c_clk_div(&mut self) -> I2cClkDivW<'_, I2cCfg0Spec> {
        I2cClkDivW::new(self, 16)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn i2c_clk_en(&mut self) -> I2cClkEnW<'_, I2cCfg0Spec> {
        I2cClkEnW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn i2c_clk_sel(&mut self) -> I2cClkSelW<'_, I2cCfg0Spec> {
        I2cClkSelW::new(self, 25)
    }
}
#[doc = "I2C Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cCfg0Spec;
impl crate::RegisterSpec for I2cCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_cfg0::R`](R) reader structure"]
impl crate::Readable for I2cCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_cfg0::W`](W) writer structure"]
impl crate::Writable for I2cCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_CFG0 to value 0"]
impl crate::Resettable for I2cCfg0Spec {}
