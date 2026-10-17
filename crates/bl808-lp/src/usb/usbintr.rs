#[doc = "Register `USBINTR` reader"]
pub type R = crate::R<UsbintrSpec>;
#[doc = "Register `USBINTR` writer"]
pub type W = crate::W<UsbintrSpec>;
#[doc = "Field `USB_INT_EN` reader - "]
pub type UsbIntEnR = crate::BitReader;
#[doc = "Field `USB_INT_EN` writer - "]
pub type UsbIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBERR_INT_EN` reader - "]
pub type UsberrIntEnR = crate::BitReader;
#[doc = "Field `USBERR_INT_EN` writer - "]
pub type UsberrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PO_CHG_INT_EN` reader - "]
pub type PoChgIntEnR = crate::BitReader;
#[doc = "Field `PO_CHG_INT_EN` writer - "]
pub type PoChgIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRL_ROL_EN` reader - "]
pub type FrlRolEnR = crate::BitReader;
#[doc = "Field `FRL_ROL_EN` writer - "]
pub type FrlRolEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_SYSERR_EN` reader - "]
pub type HSyserrEnR = crate::BitReader;
#[doc = "Field `H_SYSERR_EN` writer - "]
pub type HSyserrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_OAA_EN` reader - "]
pub type IntOaaEnR = crate::BitReader;
#[doc = "Field `INT_OAA_EN` writer - "]
pub type IntOaaEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn usb_int_en(&self) -> UsbIntEnR {
        UsbIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn usberr_int_en(&self) -> UsberrIntEnR {
        UsberrIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn po_chg_int_en(&self) -> PoChgIntEnR {
        PoChgIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn frl_rol_en(&self) -> FrlRolEnR {
        FrlRolEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn h_syserr_en(&self) -> HSyserrEnR {
        HSyserrEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn int_oaa_en(&self) -> IntOaaEnR {
        IntOaaEnR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn usb_int_en(&mut self) -> UsbIntEnW<'_, UsbintrSpec> {
        UsbIntEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn usberr_int_en(&mut self) -> UsberrIntEnW<'_, UsbintrSpec> {
        UsberrIntEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn po_chg_int_en(&mut self) -> PoChgIntEnW<'_, UsbintrSpec> {
        PoChgIntEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn frl_rol_en(&mut self) -> FrlRolEnW<'_, UsbintrSpec> {
        FrlRolEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn h_syserr_en(&mut self) -> HSyserrEnW<'_, UsbintrSpec> {
        HSyserrEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn int_oaa_en(&mut self) -> IntOaaEnW<'_, UsbintrSpec> {
        IntOaaEnW::new(self, 5)
    }
}
#[doc = "USB Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbintr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbintr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbintrSpec;
impl crate::RegisterSpec for UsbintrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbintr::R`](R) reader structure"]
impl crate::Readable for UsbintrSpec {}
#[doc = "`write(|w| ..)` method takes [`usbintr::W`](W) writer structure"]
impl crate::Writable for UsbintrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USBINTR to value 0"]
impl crate::Resettable for UsbintrSpec {}
