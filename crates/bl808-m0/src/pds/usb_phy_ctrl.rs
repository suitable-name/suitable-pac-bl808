#[doc = "Register `USB_PHY_CTRL` reader"]
pub type R = crate::R<UsbPhyCtrlSpec>;
#[doc = "Register `USB_PHY_CTRL` writer"]
pub type W = crate::W<UsbPhyCtrlSpec>;
#[doc = "Field `reg_usb_phy_ponrst` reader - "]
pub type RegUsbPhyPonrstR = crate::BitReader;
#[doc = "Field `reg_usb_phy_ponrst` writer - "]
pub type RegUsbPhyPonrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_usb_phy_oscouten` reader - "]
pub type RegUsbPhyOscoutenR = crate::BitReader;
#[doc = "Field `reg_usb_phy_oscouten` writer - "]
pub type RegUsbPhyOscoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_usb_phy_xtlsel` reader - "]
pub type RegUsbPhyXtlselR = crate::FieldReader;
#[doc = "Field `reg_usb_phy_xtlsel` writer - "]
pub type RegUsbPhyXtlselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_usb_phy_outclksel` reader - "]
pub type RegUsbPhyOutclkselR = crate::BitReader;
#[doc = "Field `reg_usb_phy_outclksel` writer - "]
pub type RegUsbPhyOutclkselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_usb_phy_pllaliv` reader - "]
pub type RegUsbPhyPllalivR = crate::BitReader;
#[doc = "Field `reg_usb_phy_pllaliv` writer - "]
pub type RegUsbPhyPllalivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_pu_usb20_psw` reader - "]
pub type RegPuUsb20PswR = crate::BitReader;
#[doc = "Field `reg_pu_usb20_psw` writer - "]
pub type RegPuUsb20PswW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_usb_phy_ponrst(&self) -> RegUsbPhyPonrstR {
        RegUsbPhyPonrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_usb_phy_oscouten(&self) -> RegUsbPhyOscoutenR {
        RegUsbPhyOscoutenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_usb_phy_xtlsel(&self) -> RegUsbPhyXtlselR {
        RegUsbPhyXtlselR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_usb_phy_outclksel(&self) -> RegUsbPhyOutclkselR {
        RegUsbPhyOutclkselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_usb_phy_pllaliv(&self) -> RegUsbPhyPllalivR {
        RegUsbPhyPllalivR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_pu_usb20_psw(&self) -> RegPuUsb20PswR {
        RegPuUsb20PswR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_usb_phy_ponrst(&mut self) -> RegUsbPhyPonrstW<'_, UsbPhyCtrlSpec> {
        RegUsbPhyPonrstW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_usb_phy_oscouten(&mut self) -> RegUsbPhyOscoutenW<'_, UsbPhyCtrlSpec> {
        RegUsbPhyOscoutenW::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_usb_phy_xtlsel(&mut self) -> RegUsbPhyXtlselW<'_, UsbPhyCtrlSpec> {
        RegUsbPhyXtlselW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_usb_phy_outclksel(&mut self) -> RegUsbPhyOutclkselW<'_, UsbPhyCtrlSpec> {
        RegUsbPhyOutclkselW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_usb_phy_pllaliv(&mut self) -> RegUsbPhyPllalivW<'_, UsbPhyCtrlSpec> {
        RegUsbPhyPllalivW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_pu_usb20_psw(&mut self) -> RegPuUsb20PswW<'_, UsbPhyCtrlSpec> {
        RegPuUsb20PswW::new(self, 6)
    }
}
#[doc = "USB PHY analog control\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_phy_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_phy_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbPhyCtrlSpec;
impl crate::RegisterSpec for UsbPhyCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_phy_ctrl::R`](R) reader structure"]
impl crate::Readable for UsbPhyCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`usb_phy_ctrl::W`](W) writer structure"]
impl crate::Writable for UsbPhyCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB_PHY_CTRL to value 0"]
impl crate::Resettable for UsbPhyCtrlSpec {}
