#[doc = "Register `GPIO_CFG142` reader"]
pub type R = crate::R<GpioCfg142Spec>;
#[doc = "Register `GPIO_CFG142` writer"]
pub type W = crate::W<GpioCfg142Spec>;
#[doc = "Field `cr_gpio_tx_en` reader - "]
pub type CrGpioTxEnR = crate::BitReader;
#[doc = "Field `cr_gpio_tx_en` writer - "]
pub type CrGpioTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_invert_code0_high` reader - "]
pub type CrInvertCode0HighR = crate::BitReader;
#[doc = "Field `cr_invert_code0_high` writer - "]
pub type CrInvertCode0HighW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_invert_code1_high` reader - "]
pub type CrInvertCode1HighR = crate::BitReader;
#[doc = "Field `cr_invert_code1_high` writer - "]
pub type CrInvertCode1HighW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_code_total_time` reader - "]
pub type CrCodeTotalTimeR = crate::FieldReader<u16>;
#[doc = "Field `cr_code_total_time` writer - "]
pub type CrCodeTotalTimeW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `cr_code0_high_time` reader - "]
pub type CrCode0HighTimeR = crate::FieldReader;
#[doc = "Field `cr_code0_high_time` writer - "]
pub type CrCode0HighTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_code1_high_time` reader - "]
pub type CrCode1HighTimeR = crate::FieldReader;
#[doc = "Field `cr_code1_high_time` writer - "]
pub type CrCode1HighTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_gpio_tx_en(&self) -> CrGpioTxEnR {
        CrGpioTxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_invert_code0_high(&self) -> CrInvertCode0HighR {
        CrInvertCode0HighR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_invert_code1_high(&self) -> CrInvertCode1HighR {
        CrInvertCode1HighR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 7:15"]
    #[inline(always)]
    pub fn cr_code_total_time(&self) -> CrCodeTotalTimeR {
        CrCodeTotalTimeR::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_code0_high_time(&self) -> CrCode0HighTimeR {
        CrCode0HighTimeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_code1_high_time(&self) -> CrCode1HighTimeR {
        CrCode1HighTimeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_gpio_tx_en(&mut self) -> CrGpioTxEnW<'_, GpioCfg142Spec> {
        CrGpioTxEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_invert_code0_high(&mut self) -> CrInvertCode0HighW<'_, GpioCfg142Spec> {
        CrInvertCode0HighW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_invert_code1_high(&mut self) -> CrInvertCode1HighW<'_, GpioCfg142Spec> {
        CrInvertCode1HighW::new(self, 2)
    }
    #[doc = "Bits 7:15"]
    #[inline(always)]
    pub fn cr_code_total_time(&mut self) -> CrCodeTotalTimeW<'_, GpioCfg142Spec> {
        CrCodeTotalTimeW::new(self, 7)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_code0_high_time(&mut self) -> CrCode0HighTimeW<'_, GpioCfg142Spec> {
        CrCode0HighTimeW::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_code1_high_time(&mut self) -> CrCode1HighTimeW<'_, GpioCfg142Spec> {
        CrCode1HighTimeW::new(self, 24)
    }
}
#[doc = "GPIO TX Protocol Engine Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfg142::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfg142::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfg142Spec;
impl crate::RegisterSpec for GpioCfg142Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfg142::R`](R) reader structure"]
impl crate::Readable for GpioCfg142Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfg142::W`](W) writer structure"]
impl crate::Writable for GpioCfg142Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFG142 to value 0"]
impl crate::Resettable for GpioCfg142Spec {}
