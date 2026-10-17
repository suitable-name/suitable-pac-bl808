#[doc = "Register `I2C_PRD_DATA` reader"]
pub type R = crate::R<I2cPrdDataSpec>;
#[doc = "Register `I2C_PRD_DATA` writer"]
pub type W = crate::W<I2cPrdDataSpec>;
#[doc = "Field `cr_i2c_prd_d_ph_0` reader - "]
pub type CrI2cPrdDPh0R = crate::FieldReader;
#[doc = "Field `cr_i2c_prd_d_ph_0` writer - "]
pub type CrI2cPrdDPh0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_i2c_prd_d_ph_1` reader - "]
pub type CrI2cPrdDPh1R = crate::FieldReader;
#[doc = "Field `cr_i2c_prd_d_ph_1` writer - "]
pub type CrI2cPrdDPh1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_i2c_prd_d_ph_2` reader - "]
pub type CrI2cPrdDPh2R = crate::FieldReader;
#[doc = "Field `cr_i2c_prd_d_ph_2` writer - "]
pub type CrI2cPrdDPh2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_i2c_prd_d_ph_3` reader - "]
pub type CrI2cPrdDPh3R = crate::FieldReader;
#[doc = "Field `cr_i2c_prd_d_ph_3` writer - "]
pub type CrI2cPrdDPh3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_i2c_prd_d_ph_0(&self) -> CrI2cPrdDPh0R {
        CrI2cPrdDPh0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn cr_i2c_prd_d_ph_1(&self) -> CrI2cPrdDPh1R {
        CrI2cPrdDPh1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_i2c_prd_d_ph_2(&self) -> CrI2cPrdDPh2R {
        CrI2cPrdDPh2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_i2c_prd_d_ph_3(&self) -> CrI2cPrdDPh3R {
        CrI2cPrdDPh3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_i2c_prd_d_ph_0(&mut self) -> CrI2cPrdDPh0W<'_, I2cPrdDataSpec> {
        CrI2cPrdDPh0W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn cr_i2c_prd_d_ph_1(&mut self) -> CrI2cPrdDPh1W<'_, I2cPrdDataSpec> {
        CrI2cPrdDPh1W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_i2c_prd_d_ph_2(&mut self) -> CrI2cPrdDPh2W<'_, I2cPrdDataSpec> {
        CrI2cPrdDPh2W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_i2c_prd_d_ph_3(&mut self) -> CrI2cPrdDPh3W<'_, I2cPrdDataSpec> {
        CrI2cPrdDPh3W::new(self, 24)
    }
}
#[doc = "Data Phase Timing\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_prd_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_prd_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cPrdDataSpec;
impl crate::RegisterSpec for I2cPrdDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_prd_data::R`](R) reader structure"]
impl crate::Readable for I2cPrdDataSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_prd_data::W`](W) writer structure"]
impl crate::Writable for I2cPrdDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_PRD_DATA to value 0x0f0f_0f0f"]
impl crate::Resettable for I2cPrdDataSpec {
    const RESET_VALUE: u32 = 0x0f0f_0f0f;
}
