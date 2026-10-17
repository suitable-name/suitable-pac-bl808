#[doc = "Register `HBN_PAD_CTRL_0` reader"]
pub type R = crate::R<HbnPadCtrl0Spec>;
#[doc = "Register `HBN_PAD_CTRL_0` writer"]
pub type W = crate::W<HbnPadCtrl0Spec>;
#[doc = "Field `reg_aon_pad_ie_smt` reader - "]
pub type RegAonPadIeSmtR = crate::FieldReader<u16>;
#[doc = "Field `reg_aon_pad_ie_smt` writer - "]
pub type RegAonPadIeSmtW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `reg_aon_led_sel` reader - "]
pub type RegAonLedSelR = crate::FieldReader<u16>;
#[doc = "Field `reg_aon_led_sel` writer - "]
pub type RegAonLedSelW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `reg_en_aon_ctrl_gpio` reader - "]
pub type RegEnAonCtrlGpioR = crate::FieldReader<u16>;
#[doc = "Field `reg_en_aon_ctrl_gpio` writer - "]
pub type RegEnAonCtrlGpioW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `reg_aon_gpio_iso_mode` reader - "]
pub type RegAonGpioIsoModeR = crate::BitReader;
#[doc = "Field `reg_aon_gpio_iso_mode` writer - "]
pub type RegAonGpioIsoModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn reg_aon_pad_ie_smt(&self) -> RegAonPadIeSmtR {
        RegAonPadIeSmtR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 10:18"]
    #[inline(always)]
    pub fn reg_aon_led_sel(&self) -> RegAonLedSelR {
        RegAonLedSelR::new(((self.bits >> 10) & 0x01ff) as u16)
    }
    #[doc = "Bits 20:28"]
    #[inline(always)]
    pub fn reg_en_aon_ctrl_gpio(&self) -> RegEnAonCtrlGpioR {
        RegEnAonCtrlGpioR::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_aon_gpio_iso_mode(&self) -> RegAonGpioIsoModeR {
        RegAonGpioIsoModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn reg_aon_pad_ie_smt(&mut self) -> RegAonPadIeSmtW<'_, HbnPadCtrl0Spec> {
        RegAonPadIeSmtW::new(self, 0)
    }
    #[doc = "Bits 10:18"]
    #[inline(always)]
    pub fn reg_aon_led_sel(&mut self) -> RegAonLedSelW<'_, HbnPadCtrl0Spec> {
        RegAonLedSelW::new(self, 10)
    }
    #[doc = "Bits 20:28"]
    #[inline(always)]
    pub fn reg_en_aon_ctrl_gpio(&mut self) -> RegEnAonCtrlGpioW<'_, HbnPadCtrl0Spec> {
        RegEnAonCtrlGpioW::new(self, 20)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_aon_gpio_iso_mode(&mut self) -> RegAonGpioIsoModeW<'_, HbnPadCtrl0Spec> {
        RegAonGpioIsoModeW::new(self, 31)
    }
}
#[doc = "AON GPIO Pad Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_pad_ctrl_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_pad_ctrl_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HbnPadCtrl0Spec;
impl crate::RegisterSpec for HbnPadCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_pad_ctrl_0::R`](R) reader structure"]
impl crate::Readable for HbnPadCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`hbn_pad_ctrl_0::W`](W) writer structure"]
impl crate::Writable for HbnPadCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HBN_PAD_CTRL_0 to value 0x1800_0000"]
impl crate::Resettable for HbnPadCtrl0Spec {
    const RESET_VALUE: u32 = 0x1800_0000;
}
