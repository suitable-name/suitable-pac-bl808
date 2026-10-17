#[doc = "Register `USBSTS` reader"]
pub type R = crate::R<UsbstsSpec>;
#[doc = "Register `USBSTS` writer"]
pub type W = crate::W<UsbstsSpec>;
#[doc = "Field `USB_INT` reader - USB Interrupt (Ith)"]
pub type UsbIntR = crate::BitReader;
#[doc = "Field `USB_INT` writer - USB Interrupt (Ith)"]
pub type UsbIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `USBERR_INT` reader - USB Error Interrupt"]
pub type UsberrIntR = crate::BitReader;
#[doc = "Field `USBERR_INT` writer - USB Error Interrupt"]
pub type UsberrIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PO_CHG_DET` reader - Port Change Detect"]
pub type PoChgDetR = crate::BitReader;
#[doc = "Field `PO_CHG_DET` writer - Port Change Detect"]
pub type PoChgDetW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FRL_ROL` reader - Frame List Rollover"]
pub type FrlRolR = crate::BitReader;
#[doc = "Field `FRL_ROL` writer - Frame List Rollover"]
pub type FrlRolW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `H_SYSERR` reader - Host System Error"]
pub type HSyserrR = crate::BitReader;
#[doc = "Field `H_SYSERR` writer - Host System Error"]
pub type HSyserrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INT_OAA` reader - Interrupt On Asynchronous Advance"]
pub type IntOaaR = crate::BitReader;
#[doc = "Field `INT_OAA` writer - Interrupt On Asynchronous Advance"]
pub type IntOaaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `HCHALTED` reader - Host Controller Halted"]
pub type HchaltedR = crate::BitReader;
#[doc = "Field `RECLAMATION` reader - Reclamation Process Active"]
pub type ReclamationR = crate::BitReader;
#[doc = "Field `PSCH_STS` reader - Periodic Schedule Status"]
pub type PschStsR = crate::BitReader;
#[doc = "Field `ASCH_STS` reader - Asynchronous Schedule Status"]
pub type AschStsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - USB Interrupt (Ith)"]
    #[inline(always)]
    pub fn usb_int(&self) -> UsbIntR {
        UsbIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB Error Interrupt"]
    #[inline(always)]
    pub fn usberr_int(&self) -> UsberrIntR {
        UsberrIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Change Detect"]
    #[inline(always)]
    pub fn po_chg_det(&self) -> PoChgDetR {
        PoChgDetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame List Rollover"]
    #[inline(always)]
    pub fn frl_rol(&self) -> FrlRolR {
        FrlRolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Host System Error"]
    #[inline(always)]
    pub fn h_syserr(&self) -> HSyserrR {
        HSyserrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt On Asynchronous Advance"]
    #[inline(always)]
    pub fn int_oaa(&self) -> IntOaaR {
        IntOaaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - Host Controller Halted"]
    #[inline(always)]
    pub fn hchalted(&self) -> HchaltedR {
        HchaltedR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reclamation Process Active"]
    #[inline(always)]
    pub fn reclamation(&self) -> ReclamationR {
        ReclamationR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Periodic Schedule Status"]
    #[inline(always)]
    pub fn psch_sts(&self) -> PschStsR {
        PschStsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Asynchronous Schedule Status"]
    #[inline(always)]
    pub fn asch_sts(&self) -> AschStsR {
        AschStsR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Interrupt (Ith)"]
    #[inline(always)]
    pub fn usb_int(&mut self) -> UsbIntW<'_, UsbstsSpec> {
        UsbIntW::new(self, 0)
    }
    #[doc = "Bit 1 - USB Error Interrupt"]
    #[inline(always)]
    pub fn usberr_int(&mut self) -> UsberrIntW<'_, UsbstsSpec> {
        UsberrIntW::new(self, 1)
    }
    #[doc = "Bit 2 - Port Change Detect"]
    #[inline(always)]
    pub fn po_chg_det(&mut self) -> PoChgDetW<'_, UsbstsSpec> {
        PoChgDetW::new(self, 2)
    }
    #[doc = "Bit 3 - Frame List Rollover"]
    #[inline(always)]
    pub fn frl_rol(&mut self) -> FrlRolW<'_, UsbstsSpec> {
        FrlRolW::new(self, 3)
    }
    #[doc = "Bit 4 - Host System Error"]
    #[inline(always)]
    pub fn h_syserr(&mut self) -> HSyserrW<'_, UsbstsSpec> {
        HSyserrW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt On Asynchronous Advance"]
    #[inline(always)]
    pub fn int_oaa(&mut self) -> IntOaaW<'_, UsbstsSpec> {
        IntOaaW::new(self, 5)
    }
}
#[doc = "USB Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbstsSpec;
impl crate::RegisterSpec for UsbstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbsts::R`](R) reader structure"]
impl crate::Readable for UsbstsSpec {}
#[doc = "`write(|w| ..)` method takes [`usbsts::W`](W) writer structure"]
impl crate::Writable for UsbstsSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets USBSTS to value 0"]
impl crate::Resettable for UsbstsSpec {}
