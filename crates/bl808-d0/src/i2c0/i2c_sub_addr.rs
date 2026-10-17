#[doc = "Register `I2C_SUB_ADDR` reader"]
pub type R = crate::R<I2cSubAddrSpec>;
#[doc = "Register `I2C_SUB_ADDR` writer"]
pub type W = crate::W<I2cSubAddrSpec>;
#[doc = "Field `cr_i2c_sub_addr_b0` reader - Sub-address byte 0"]
pub type CrI2cSubAddrB0R = crate::FieldReader;
#[doc = "Field `cr_i2c_sub_addr_b0` writer - Sub-address byte 0"]
pub type CrI2cSubAddrB0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_i2c_sub_addr_b1` reader - Sub-address byte 1"]
pub type CrI2cSubAddrB1R = crate::FieldReader;
#[doc = "Field `cr_i2c_sub_addr_b1` writer - Sub-address byte 1"]
pub type CrI2cSubAddrB1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_i2c_sub_addr_b2` reader - Sub-address byte 2"]
pub type CrI2cSubAddrB2R = crate::FieldReader;
#[doc = "Field `cr_i2c_sub_addr_b2` writer - Sub-address byte 2"]
pub type CrI2cSubAddrB2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_i2c_sub_addr_b3` reader - Sub-address byte 3"]
pub type CrI2cSubAddrB3R = crate::FieldReader;
#[doc = "Field `cr_i2c_sub_addr_b3` writer - Sub-address byte 3"]
pub type CrI2cSubAddrB3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sub-address byte 0"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_b0(&self) -> CrI2cSubAddrB0R {
        CrI2cSubAddrB0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sub-address byte 1"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_b1(&self) -> CrI2cSubAddrB1R {
        CrI2cSubAddrB1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sub-address byte 2"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_b2(&self) -> CrI2cSubAddrB2R {
        CrI2cSubAddrB2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Sub-address byte 3"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_b3(&self) -> CrI2cSubAddrB3R {
        CrI2cSubAddrB3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sub-address byte 0"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_b0(&mut self) -> CrI2cSubAddrB0W<'_, I2cSubAddrSpec> {
        CrI2cSubAddrB0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Sub-address byte 1"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_b1(&mut self) -> CrI2cSubAddrB1W<'_, I2cSubAddrSpec> {
        CrI2cSubAddrB1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Sub-address byte 2"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_b2(&mut self) -> CrI2cSubAddrB2W<'_, I2cSubAddrSpec> {
        CrI2cSubAddrB2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Sub-address byte 3"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_b3(&mut self) -> CrI2cSubAddrB3W<'_, I2cSubAddrSpec> {
        CrI2cSubAddrB3W::new(self, 24)
    }
}
#[doc = "I2C Sub-address Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_sub_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_sub_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cSubAddrSpec;
impl crate::RegisterSpec for I2cSubAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_sub_addr::R`](R) reader structure"]
impl crate::Readable for I2cSubAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_sub_addr::W`](W) writer structure"]
impl crate::Writable for I2cSubAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_SUB_ADDR to value 0"]
impl crate::Resettable for I2cSubAddrSpec {}
