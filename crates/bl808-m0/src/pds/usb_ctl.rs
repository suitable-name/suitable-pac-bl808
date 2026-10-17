#[doc = "Register `USB_CTL` reader"]
pub type R = crate::R<UsbCtlSpec>;
#[doc = "Register `USB_CTL` writer"]
pub type W = crate::W<UsbCtlSpec>;
#[doc = "Field `reg_usb_sw_rst_n` reader - "]
pub type RegUsbSwRstNR = crate::BitReader;
#[doc = "Field `reg_usb_sw_rst_n` writer - "]
pub type RegUsbSwRstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_usb_ext_susp_n` reader - "]
pub type RegUsbExtSuspNR = crate::BitReader;
#[doc = "Field `reg_usb_ext_susp_n` writer - "]
pub type RegUsbExtSuspNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_usb_wakeup` reader - "]
pub type RegUsbWakeupR = crate::BitReader;
#[doc = "Field `reg_usb_wakeup` writer - "]
pub type RegUsbWakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_usb_l1_wakeup` reader - "]
pub type RegUsbL1WakeupR = crate::BitReader;
#[doc = "Field `reg_usb_l1_wakeup` writer - "]
pub type RegUsbL1WakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_usb_drvbus_pol` reader - "]
pub type RegUsbDrvbusPolR = crate::BitReader;
#[doc = "Field `reg_usb_drvbus_pol` writer - "]
pub type RegUsbDrvbusPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_usb_iddig` reader - "]
pub type RegUsbIddigR = crate::BitReader;
#[doc = "Field `reg_usb_iddig` writer - "]
pub type RegUsbIddigW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_usb_sw_rst_n(&self) -> RegUsbSwRstNR {
        RegUsbSwRstNR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_usb_ext_susp_n(&self) -> RegUsbExtSuspNR {
        RegUsbExtSuspNR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_usb_wakeup(&self) -> RegUsbWakeupR {
        RegUsbWakeupR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_usb_l1_wakeup(&self) -> RegUsbL1WakeupR {
        RegUsbL1WakeupR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_usb_drvbus_pol(&self) -> RegUsbDrvbusPolR {
        RegUsbDrvbusPolR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_usb_iddig(&self) -> RegUsbIddigR {
        RegUsbIddigR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_usb_sw_rst_n(&mut self) -> RegUsbSwRstNW<'_, UsbCtlSpec> {
        RegUsbSwRstNW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_usb_ext_susp_n(&mut self) -> RegUsbExtSuspNW<'_, UsbCtlSpec> {
        RegUsbExtSuspNW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_usb_wakeup(&mut self) -> RegUsbWakeupW<'_, UsbCtlSpec> {
        RegUsbWakeupW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_usb_l1_wakeup(&mut self) -> RegUsbL1WakeupW<'_, UsbCtlSpec> {
        RegUsbL1WakeupW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_usb_drvbus_pol(&mut self) -> RegUsbDrvbusPolW<'_, UsbCtlSpec> {
        RegUsbDrvbusPolW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_usb_iddig(&mut self) -> RegUsbIddigW<'_, UsbCtlSpec> {
        RegUsbIddigW::new(self, 5)
    }
}
#[doc = "USB top-level software control\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbCtlSpec;
impl crate::RegisterSpec for UsbCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_ctl::R`](R) reader structure"]
impl crate::Readable for UsbCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`usb_ctl::W`](W) writer structure"]
impl crate::Writable for UsbCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB_CTL to value 0x21"]
impl crate::Resettable for UsbCtlSpec {
    const RESET_VALUE: u32 = 0x21;
}
