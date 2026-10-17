#[doc = "Register `I2C_BUS_BUSY` reader"]
pub type R = crate::R<I2cBusBusySpec>;
#[doc = "Register `I2C_BUS_BUSY` writer"]
pub type W = crate::W<I2cBusBusySpec>;
#[doc = "Field `sts_i2c_bus_busy` reader - I2C bus busy status"]
pub type StsI2cBusBusyR = crate::BitReader;
#[doc = "Field `cr_i2c_bus_busy_clr` writer - Clear bus busy status"]
pub type CrI2cBusBusyClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C bus busy status"]
    #[inline(always)]
    pub fn sts_i2c_bus_busy(&self) -> StsI2cBusBusyR {
        StsI2cBusBusyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Clear bus busy status"]
    #[inline(always)]
    pub fn cr_i2c_bus_busy_clr(&mut self) -> CrI2cBusBusyClrW<'_, I2cBusBusySpec> {
        CrI2cBusBusyClrW::new(self, 1)
    }
}
#[doc = "I2C Bus Status\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_bus_busy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_bus_busy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cBusBusySpec;
impl crate::RegisterSpec for I2cBusBusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_bus_busy::R`](R) reader structure"]
impl crate::Readable for I2cBusBusySpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_bus_busy::W`](W) writer structure"]
impl crate::Writable for I2cBusBusySpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x02;
}
#[doc = "`reset()` method sets I2C_BUS_BUSY to value 0"]
impl crate::Resettable for I2cBusBusySpec {}
