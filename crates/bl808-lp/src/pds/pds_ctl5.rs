#[doc = "Register `PDS_CTL5` reader"]
pub type R = crate::R<PdsCtl5Spec>;
#[doc = "Register `PDS_CTL5` writer"]
pub type W = crate::W<PdsCtl5Spec>;
#[doc = "Field `cr_np_wfi_mask` reader - "]
pub type CrNpWfiMaskR = crate::BitReader;
#[doc = "Field `cr_np_wfi_mask` writer - "]
pub type CrNpWfiMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_mm_wfi_mask` reader - "]
pub type CrMmWfiMaskR = crate::BitReader;
#[doc = "Field `cr_mm_wfi_mask` writer - "]
pub type CrMmWfiMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pico_wfi_mask` reader - "]
pub type CrPicoWfiMaskR = crate::BitReader;
#[doc = "Field `cr_pico_wfi_mask` writer - "]
pub type CrPicoWfiMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_ctrl_usb33` reader - "]
pub type CrPdsCtrlUsb33R = crate::BitReader;
#[doc = "Field `cr_pds_ctrl_usb33` writer - "]
pub type CrPdsCtrlUsb33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_pd_ldo18io` reader - "]
pub type CrPdsPdLdo18ioR = crate::BitReader;
#[doc = "Field `cr_pds_pd_ldo18io` writer - "]
pub type CrPdsPdLdo18ioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_gpio_keep_en` reader - "]
pub type CrPdsGpioKeepEnR = crate::FieldReader;
#[doc = "Field `cr_pds_gpio_keep_en` writer - "]
pub type CrPdsGpioKeepEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_np_wfi_mask(&self) -> CrNpWfiMaskR {
        CrNpWfiMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_mm_wfi_mask(&self) -> CrMmWfiMaskR {
        CrMmWfiMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pico_wfi_mask(&self) -> CrPicoWfiMaskR {
        CrPicoWfiMaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_ctrl_usb33(&self) -> CrPdsCtrlUsb33R {
        CrPdsCtrlUsb33R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_pd_ldo18io(&self) -> CrPdsPdLdo18ioR {
        CrPdsPdLdo18ioR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn cr_pds_gpio_keep_en(&self) -> CrPdsGpioKeepEnR {
        CrPdsGpioKeepEnR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_np_wfi_mask(&mut self) -> CrNpWfiMaskW<'_, PdsCtl5Spec> {
        CrNpWfiMaskW::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_mm_wfi_mask(&mut self) -> CrMmWfiMaskW<'_, PdsCtl5Spec> {
        CrMmWfiMaskW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pico_wfi_mask(&mut self) -> CrPicoWfiMaskW<'_, PdsCtl5Spec> {
        CrPicoWfiMaskW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_ctrl_usb33(&mut self) -> CrPdsCtrlUsb33W<'_, PdsCtl5Spec> {
        CrPdsCtrlUsb33W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_pd_ldo18io(&mut self) -> CrPdsPdLdo18ioW<'_, PdsCtl5Spec> {
        CrPdsPdLdo18ioW::new(self, 9)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn cr_pds_gpio_keep_en(&mut self) -> CrPdsGpioKeepEnW<'_, PdsCtl5Spec> {
        CrPdsGpioKeepEnW::new(self, 16)
    }
}
#[doc = "PDS Control Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_ctl5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_ctl5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdsCtl5Spec;
impl crate::RegisterSpec for PdsCtl5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_ctl5::R`](R) reader structure"]
impl crate::Readable for PdsCtl5Spec {}
#[doc = "`write(|w| ..)` method takes [`pds_ctl5::W`](W) writer structure"]
impl crate::Writable for PdsCtl5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDS_CTL5 to value 0x0007_0000"]
impl crate::Resettable for PdsCtl5Spec {
    const RESET_VALUE: u32 = 0x0007_0000;
}
