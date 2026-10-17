#[doc = "Register `I2C_PRD_STOP` reader"]
pub type R = crate::R<I2cPrdStopSpec>;
#[doc = "Register `I2C_PRD_STOP` writer"]
pub type W = crate::W<I2cPrdStopSpec>;
#[doc = "Field `cr_i2c_prd_p_ph_0` reader - "]
pub type CrI2cPrdPPh0R = crate::FieldReader;
#[doc = "Field `cr_i2c_prd_p_ph_0` writer - "]
pub type CrI2cPrdPPh0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_i2c_prd_p_ph_1` reader - "]
pub type CrI2cPrdPPh1R = crate::FieldReader;
#[doc = "Field `cr_i2c_prd_p_ph_1` writer - "]
pub type CrI2cPrdPPh1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_i2c_prd_p_ph_2` reader - "]
pub type CrI2cPrdPPh2R = crate::FieldReader;
#[doc = "Field `cr_i2c_prd_p_ph_2` writer - "]
pub type CrI2cPrdPPh2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_i2c_prd_p_ph_3` reader - "]
pub type CrI2cPrdPPh3R = crate::FieldReader;
#[doc = "Field `cr_i2c_prd_p_ph_3` writer - "]
pub type CrI2cPrdPPh3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_i2c_prd_p_ph_0(&self) -> CrI2cPrdPPh0R {
        CrI2cPrdPPh0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn cr_i2c_prd_p_ph_1(&self) -> CrI2cPrdPPh1R {
        CrI2cPrdPPh1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_i2c_prd_p_ph_2(&self) -> CrI2cPrdPPh2R {
        CrI2cPrdPPh2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_i2c_prd_p_ph_3(&self) -> CrI2cPrdPPh3R {
        CrI2cPrdPPh3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_i2c_prd_p_ph_0(&mut self) -> CrI2cPrdPPh0W<'_, I2cPrdStopSpec> {
        CrI2cPrdPPh0W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn cr_i2c_prd_p_ph_1(&mut self) -> CrI2cPrdPPh1W<'_, I2cPrdStopSpec> {
        CrI2cPrdPPh1W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_i2c_prd_p_ph_2(&mut self) -> CrI2cPrdPPh2W<'_, I2cPrdStopSpec> {
        CrI2cPrdPPh2W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_i2c_prd_p_ph_3(&mut self) -> CrI2cPrdPPh3W<'_, I2cPrdStopSpec> {
        CrI2cPrdPPh3W::new(self, 24)
    }
}
#[doc = "Stop Condition Timing Phases\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_prd_stop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_prd_stop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cPrdStopSpec;
impl crate::RegisterSpec for I2cPrdStopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_prd_stop::R`](R) reader structure"]
impl crate::Readable for I2cPrdStopSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_prd_stop::W`](W) writer structure"]
impl crate::Writable for I2cPrdStopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_PRD_STOP to value 0x0f0f_0f0f"]
impl crate::Resettable for I2cPrdStopSpec {
    const RESET_VALUE: u32 = 0x0f0f_0f0f;
}
