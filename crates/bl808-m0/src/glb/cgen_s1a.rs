#[doc = "Register `CGEN_S1A` reader"]
pub type R = crate::R<CgenS1aSpec>;
#[doc = "Register `CGEN_S1A` writer"]
pub type W = crate::W<CgenS1aSpec>;
#[doc = "Field `cgen_s1_gpip` reader - "]
pub type CgenS1GpipR = crate::BitReader;
#[doc = "Field `cgen_s1_gpip` writer - "]
pub type CgenS1GpipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_sec_dbg` reader - "]
pub type CgenS1SecDbgR = crate::BitReader;
#[doc = "Field `cgen_s1_sec_dbg` writer - "]
pub type CgenS1SecDbgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_sec_eng` reader - "]
pub type CgenS1SecEngR = crate::BitReader;
#[doc = "Field `cgen_s1_sec_eng` writer - "]
pub type CgenS1SecEngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_tz` reader - "]
pub type CgenS1TzR = crate::BitReader;
#[doc = "Field `cgen_s1_tz` writer - "]
pub type CgenS1TzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_ef_ctrl` reader - "]
pub type CgenS1EfCtrlR = crate::BitReader;
#[doc = "Field `cgen_s1_ef_ctrl` writer - "]
pub type CgenS1EfCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_sf_ctrl` reader - "]
pub type CgenS1SfCtrlR = crate::BitReader;
#[doc = "Field `cgen_s1_sf_ctrl` writer - "]
pub type CgenS1SfCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_dma` reader - "]
pub type CgenS1DmaR = crate::BitReader;
#[doc = "Field `cgen_s1_dma` writer - "]
pub type CgenS1DmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1a_uart0` reader - "]
pub type CgenS1aUart0R = crate::BitReader;
#[doc = "Field `cgen_s1a_uart0` writer - "]
pub type CgenS1aUart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1a_uart1` reader - "]
pub type CgenS1aUart1R = crate::BitReader;
#[doc = "Field `cgen_s1a_uart1` writer - "]
pub type CgenS1aUart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1a_spi` reader - "]
pub type CgenS1aSpiR = crate::BitReader;
#[doc = "Field `cgen_s1a_spi` writer - "]
pub type CgenS1aSpiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1a_i2c` reader - "]
pub type CgenS1aI2cR = crate::BitReader;
#[doc = "Field `cgen_s1a_i2c` writer - "]
pub type CgenS1aI2cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1a_pwm` reader - "]
pub type CgenS1aPwmR = crate::BitReader;
#[doc = "Field `cgen_s1a_pwm` writer - "]
pub type CgenS1aPwmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1a_timer` reader - "]
pub type CgenS1aTimerR = crate::BitReader;
#[doc = "Field `cgen_s1a_timer` writer - "]
pub type CgenS1aTimerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1a_ir` reader - "]
pub type CgenS1aIrR = crate::BitReader;
#[doc = "Field `cgen_s1a_ir` writer - "]
pub type CgenS1aIrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1a_cks` reader - "]
pub type CgenS1aCksR = crate::BitReader;
#[doc = "Field `cgen_s1a_cks` writer - "]
pub type CgenS1aCksW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1a_i2c1` reader - "]
pub type CgenS1aI2c1R = crate::BitReader;
#[doc = "Field `cgen_s1a_i2c1` writer - "]
pub type CgenS1aI2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1a_uart2` reader - "]
pub type CgenS1aUart2R = crate::BitReader;
#[doc = "Field `cgen_s1a_uart2` writer - "]
pub type CgenS1aUart2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cgen_s1_gpip(&self) -> CgenS1GpipR {
        CgenS1GpipR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cgen_s1_sec_dbg(&self) -> CgenS1SecDbgR {
        CgenS1SecDbgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cgen_s1_sec_eng(&self) -> CgenS1SecEngR {
        CgenS1SecEngR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cgen_s1_tz(&self) -> CgenS1TzR {
        CgenS1TzR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cgen_s1_ef_ctrl(&self) -> CgenS1EfCtrlR {
        CgenS1EfCtrlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cgen_s1_sf_ctrl(&self) -> CgenS1SfCtrlR {
        CgenS1SfCtrlR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cgen_s1_dma(&self) -> CgenS1DmaR {
        CgenS1DmaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cgen_s1a_uart0(&self) -> CgenS1aUart0R {
        CgenS1aUart0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cgen_s1a_uart1(&self) -> CgenS1aUart1R {
        CgenS1aUart1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cgen_s1a_spi(&self) -> CgenS1aSpiR {
        CgenS1aSpiR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cgen_s1a_i2c(&self) -> CgenS1aI2cR {
        CgenS1aI2cR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cgen_s1a_pwm(&self) -> CgenS1aPwmR {
        CgenS1aPwmR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cgen_s1a_timer(&self) -> CgenS1aTimerR {
        CgenS1aTimerR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cgen_s1a_ir(&self) -> CgenS1aIrR {
        CgenS1aIrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cgen_s1a_cks(&self) -> CgenS1aCksR {
        CgenS1aCksR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cgen_s1a_i2c1(&self) -> CgenS1aI2c1R {
        CgenS1aI2c1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cgen_s1a_uart2(&self) -> CgenS1aUart2R {
        CgenS1aUart2R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cgen_s1_gpip(&mut self) -> CgenS1GpipW<'_, CgenS1aSpec> {
        CgenS1GpipW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cgen_s1_sec_dbg(&mut self) -> CgenS1SecDbgW<'_, CgenS1aSpec> {
        CgenS1SecDbgW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cgen_s1_sec_eng(&mut self) -> CgenS1SecEngW<'_, CgenS1aSpec> {
        CgenS1SecEngW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cgen_s1_tz(&mut self) -> CgenS1TzW<'_, CgenS1aSpec> {
        CgenS1TzW::new(self, 5)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cgen_s1_ef_ctrl(&mut self) -> CgenS1EfCtrlW<'_, CgenS1aSpec> {
        CgenS1EfCtrlW::new(self, 7)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cgen_s1_sf_ctrl(&mut self) -> CgenS1SfCtrlW<'_, CgenS1aSpec> {
        CgenS1SfCtrlW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cgen_s1_dma(&mut self) -> CgenS1DmaW<'_, CgenS1aSpec> {
        CgenS1DmaW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cgen_s1a_uart0(&mut self) -> CgenS1aUart0W<'_, CgenS1aSpec> {
        CgenS1aUart0W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cgen_s1a_uart1(&mut self) -> CgenS1aUart1W<'_, CgenS1aSpec> {
        CgenS1aUart1W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cgen_s1a_spi(&mut self) -> CgenS1aSpiW<'_, CgenS1aSpec> {
        CgenS1aSpiW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cgen_s1a_i2c(&mut self) -> CgenS1aI2cW<'_, CgenS1aSpec> {
        CgenS1aI2cW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cgen_s1a_pwm(&mut self) -> CgenS1aPwmW<'_, CgenS1aSpec> {
        CgenS1aPwmW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cgen_s1a_timer(&mut self) -> CgenS1aTimerW<'_, CgenS1aSpec> {
        CgenS1aTimerW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cgen_s1a_ir(&mut self) -> CgenS1aIrW<'_, CgenS1aSpec> {
        CgenS1aIrW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cgen_s1a_cks(&mut self) -> CgenS1aCksW<'_, CgenS1aSpec> {
        CgenS1aCksW::new(self, 23)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cgen_s1a_i2c1(&mut self) -> CgenS1aI2c1W<'_, CgenS1aSpec> {
        CgenS1aI2c1W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cgen_s1a_uart2(&mut self) -> CgenS1aUart2W<'_, CgenS1aSpec> {
        CgenS1aUart2W::new(self, 26)
    }
}
#[doc = "Clock Gate Config 1 (S1 Peripherals + S1A Peripherals)\n\nYou can [`read`](crate::Reg::read) this register and get [`cgen_s1a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgen_s1a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CgenS1aSpec;
impl crate::RegisterSpec for CgenS1aSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgen_s1a::R`](R) reader structure"]
impl crate::Readable for CgenS1aSpec {}
#[doc = "`write(|w| ..)` method takes [`cgen_s1a::W`](W) writer structure"]
impl crate::Writable for CgenS1aSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CGEN_S1A to value 0"]
impl crate::Resettable for CgenS1aSpec {}
