#[doc = "Register `GPIO` reader"]
pub type R = crate::R<GpioSpec>;
#[doc = "Register `GPIO` writer"]
pub type W = crate::W<GpioSpec>;
#[doc = "Field `timer2_gpio_en` reader - "]
pub type Timer2GpioEnR = crate::BitReader;
#[doc = "Field `timer2_gpio_en` writer - "]
pub type Timer2GpioEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `timer2_gpio_inv` reader - "]
pub type Timer2GpioInvR = crate::BitReader;
#[doc = "Field `timer2_gpio_inv` writer - "]
pub type Timer2GpioInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `timer3_gpio_inv` reader - "]
pub type Timer3GpioInvR = crate::BitReader;
#[doc = "Field `timer3_gpio_inv` writer - "]
pub type Timer3GpioInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wdt_gpio_inv` reader - "]
pub type WdtGpioInvR = crate::BitReader;
#[doc = "Field `wdt_gpio_inv` writer - "]
pub type WdtGpioInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpio_lat_ok` reader - Latch captured status"]
pub type GpioLatOkR = crate::BitReader;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer2_gpio_en(&self) -> Timer2GpioEnR {
        Timer2GpioEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn timer2_gpio_inv(&self) -> Timer2GpioInvR {
        Timer2GpioInvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn timer3_gpio_inv(&self) -> Timer3GpioInvR {
        Timer3GpioInvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn wdt_gpio_inv(&self) -> WdtGpioInvR {
        WdtGpioInvR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - Latch captured status"]
    #[inline(always)]
    pub fn gpio_lat_ok(&self) -> GpioLatOkR {
        GpioLatOkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer2_gpio_en(&mut self) -> Timer2GpioEnW<'_, GpioSpec> {
        Timer2GpioEnW::new(self, 1)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn timer2_gpio_inv(&mut self) -> Timer2GpioInvW<'_, GpioSpec> {
        Timer2GpioInvW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn timer3_gpio_inv(&mut self) -> Timer3GpioInvW<'_, GpioSpec> {
        Timer3GpioInvW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn wdt_gpio_inv(&mut self) -> WdtGpioInvW<'_, GpioSpec> {
        WdtGpioInvW::new(self, 7)
    }
}
#[doc = "GPIO Integration and Latch Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioSpec;
impl crate::RegisterSpec for GpioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio::R`](R) reader structure"]
impl crate::Readable for GpioSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio::W`](W) writer structure"]
impl crate::Writable for GpioSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO to value 0"]
impl crate::Resettable for GpioSpec {}
