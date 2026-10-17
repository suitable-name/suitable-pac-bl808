#[doc = "Register `I2C_PRD_START` reader"]
pub type R = crate::R<I2cPrdStartSpec>;
#[doc = "Register `I2C_PRD_START` writer"]
pub type W = crate::W<I2cPrdStartSpec>;
#[doc = "Field `cr_i2c_prd_s_ph_0` reader - "]
pub type CrI2cPrdSPh0R = crate::FieldReader;
#[doc = "Field `cr_i2c_prd_s_ph_0` writer - "]
pub type CrI2cPrdSPh0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_i2c_prd_s_ph_1` reader - "]
pub type CrI2cPrdSPh1R = crate::FieldReader;
#[doc = "Field `cr_i2c_prd_s_ph_1` writer - "]
pub type CrI2cPrdSPh1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_i2c_prd_s_ph_2` reader - "]
pub type CrI2cPrdSPh2R = crate::FieldReader;
#[doc = "Field `cr_i2c_prd_s_ph_2` writer - "]
pub type CrI2cPrdSPh2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_i2c_prd_s_ph_3` reader - "]
pub type CrI2cPrdSPh3R = crate::FieldReader;
#[doc = "Field `cr_i2c_prd_s_ph_3` writer - "]
pub type CrI2cPrdSPh3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_i2c_prd_s_ph_0(&self) -> CrI2cPrdSPh0R {
        CrI2cPrdSPh0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn cr_i2c_prd_s_ph_1(&self) -> CrI2cPrdSPh1R {
        CrI2cPrdSPh1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_i2c_prd_s_ph_2(&self) -> CrI2cPrdSPh2R {
        CrI2cPrdSPh2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_i2c_prd_s_ph_3(&self) -> CrI2cPrdSPh3R {
        CrI2cPrdSPh3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_i2c_prd_s_ph_0(&mut self) -> CrI2cPrdSPh0W<'_, I2cPrdStartSpec> {
        CrI2cPrdSPh0W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn cr_i2c_prd_s_ph_1(&mut self) -> CrI2cPrdSPh1W<'_, I2cPrdStartSpec> {
        CrI2cPrdSPh1W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_i2c_prd_s_ph_2(&mut self) -> CrI2cPrdSPh2W<'_, I2cPrdStartSpec> {
        CrI2cPrdSPh2W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_i2c_prd_s_ph_3(&mut self) -> CrI2cPrdSPh3W<'_, I2cPrdStartSpec> {
        CrI2cPrdSPh3W::new(self, 24)
    }
}
#[doc = "Start Condition Timing Phases\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_prd_start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_prd_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cPrdStartSpec;
impl crate::RegisterSpec for I2cPrdStartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_prd_start::R`](R) reader structure"]
impl crate::Readable for I2cPrdStartSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_prd_start::W`](W) writer structure"]
impl crate::Writable for I2cPrdStartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_PRD_START to value 0x0f0f_0f0f"]
impl crate::Resettable for I2cPrdStartSpec {
    const RESET_VALUE: u32 = 0x0f0f_0f0f;
}
