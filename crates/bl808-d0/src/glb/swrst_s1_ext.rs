#[doc = "Register `SWRST_S1_EXT` reader"]
pub type R = crate::R<SwrstS1ExtSpec>;
#[doc = "Register `SWRST_S1_EXT` writer"]
pub type W = crate::W<SwrstS1ExtSpec>;
#[doc = "Field `swrst_s00` reader - "]
pub type SwrstS00R = crate::BitReader;
#[doc = "Field `swrst_s00` writer - "]
pub type SwrstS00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_s01` reader - "]
pub type SwrstS01R = crate::BitReader;
#[doc = "Field `swrst_s01` writer - "]
pub type SwrstS01W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_s20` reader - "]
pub type SwrstS20R = crate::BitReader;
#[doc = "Field `swrst_s20` writer - "]
pub type SwrstS20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_s30` reader - "]
pub type SwrstS30R = crate::BitReader;
#[doc = "Field `swrst_s30` writer - "]
pub type SwrstS30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_s31` reader - "]
pub type SwrstS31R = crate::BitReader;
#[doc = "Field `swrst_s31` writer - "]
pub type SwrstS31W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_s32` reader - "]
pub type SwrstS32R = crate::BitReader;
#[doc = "Field `swrst_s32` writer - "]
pub type SwrstS32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_s33` reader - "]
pub type SwrstS33R = crate::BitReader;
#[doc = "Field `swrst_s33` writer - "]
pub type SwrstS33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_s1_ext_emi_misc` reader - "]
pub type SwrstS1ExtEmiMiscR = crate::BitReader;
#[doc = "Field `swrst_s1_ext_emi_misc` writer - "]
pub type SwrstS1ExtEmiMiscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_s1_ext_psram0_ctrl` reader - "]
pub type SwrstS1ExtPsram0CtrlR = crate::BitReader;
#[doc = "Field `swrst_s1_ext_psram0_ctrl` writer - "]
pub type SwrstS1ExtPsram0CtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_s1_ext_psram1_ctrl` reader - "]
pub type SwrstS1ExtPsram1CtrlR = crate::BitReader;
#[doc = "Field `swrst_s1_ext_psram1_ctrl` writer - "]
pub type SwrstS1ExtPsram1CtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_s1_ext_usb` reader - "]
pub type SwrstS1ExtUsbR = crate::BitReader;
#[doc = "Field `swrst_s1_ext_usb` writer - "]
pub type SwrstS1ExtUsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_s1_ext_mix2` reader - "]
pub type SwrstS1ExtMix2R = crate::BitReader;
#[doc = "Field `swrst_s1_ext_mix2` writer - "]
pub type SwrstS1ExtMix2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_s1_ext_audio` reader - "]
pub type SwrstS1ExtAudioR = crate::BitReader;
#[doc = "Field `swrst_s1_ext_audio` writer - "]
pub type SwrstS1ExtAudioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_s1_ext_sdh` reader - "]
pub type SwrstS1ExtSdhR = crate::BitReader;
#[doc = "Field `swrst_s1_ext_sdh` writer - "]
pub type SwrstS1ExtSdhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_s1_ext_emac` reader - "]
pub type SwrstS1ExtEmacR = crate::BitReader;
#[doc = "Field `swrst_s1_ext_emac` writer - "]
pub type SwrstS1ExtEmacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_s1_ext_dma2` reader - "]
pub type SwrstS1ExtDma2R = crate::BitReader;
#[doc = "Field `swrst_s1_ext_dma2` writer - "]
pub type SwrstS1ExtDma2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swrst_s00(&self) -> SwrstS00R {
        SwrstS00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swrst_s01(&self) -> SwrstS01R {
        SwrstS01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn swrst_s20(&self) -> SwrstS20R {
        SwrstS20R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swrst_s30(&self) -> SwrstS30R {
        SwrstS30R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn swrst_s31(&self) -> SwrstS31R {
        SwrstS31R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn swrst_s32(&self) -> SwrstS32R {
        SwrstS32R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn swrst_s33(&self) -> SwrstS33R {
        SwrstS33R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn swrst_s1_ext_emi_misc(&self) -> SwrstS1ExtEmiMiscR {
        SwrstS1ExtEmiMiscR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn swrst_s1_ext_psram0_ctrl(&self) -> SwrstS1ExtPsram0CtrlR {
        SwrstS1ExtPsram0CtrlR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn swrst_s1_ext_psram1_ctrl(&self) -> SwrstS1ExtPsram1CtrlR {
        SwrstS1ExtPsram1CtrlR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn swrst_s1_ext_usb(&self) -> SwrstS1ExtUsbR {
        SwrstS1ExtUsbR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn swrst_s1_ext_mix2(&self) -> SwrstS1ExtMix2R {
        SwrstS1ExtMix2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn swrst_s1_ext_audio(&self) -> SwrstS1ExtAudioR {
        SwrstS1ExtAudioR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn swrst_s1_ext_sdh(&self) -> SwrstS1ExtSdhR {
        SwrstS1ExtSdhR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn swrst_s1_ext_emac(&self) -> SwrstS1ExtEmacR {
        SwrstS1ExtEmacR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn swrst_s1_ext_dma2(&self) -> SwrstS1ExtDma2R {
        SwrstS1ExtDma2R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swrst_s00(&mut self) -> SwrstS00W<'_, SwrstS1ExtSpec> {
        SwrstS00W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swrst_s01(&mut self) -> SwrstS01W<'_, SwrstS1ExtSpec> {
        SwrstS01W::new(self, 1)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn swrst_s20(&mut self) -> SwrstS20W<'_, SwrstS1ExtSpec> {
        SwrstS20W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swrst_s30(&mut self) -> SwrstS30W<'_, SwrstS1ExtSpec> {
        SwrstS30W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn swrst_s31(&mut self) -> SwrstS31W<'_, SwrstS1ExtSpec> {
        SwrstS31W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn swrst_s32(&mut self) -> SwrstS32W<'_, SwrstS1ExtSpec> {
        SwrstS32W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn swrst_s33(&mut self) -> SwrstS33W<'_, SwrstS1ExtSpec> {
        SwrstS33W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn swrst_s1_ext_emi_misc(&mut self) -> SwrstS1ExtEmiMiscW<'_, SwrstS1ExtSpec> {
        SwrstS1ExtEmiMiscW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn swrst_s1_ext_psram0_ctrl(&mut self) -> SwrstS1ExtPsram0CtrlW<'_, SwrstS1ExtSpec> {
        SwrstS1ExtPsram0CtrlW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn swrst_s1_ext_psram1_ctrl(&mut self) -> SwrstS1ExtPsram1CtrlW<'_, SwrstS1ExtSpec> {
        SwrstS1ExtPsram1CtrlW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn swrst_s1_ext_usb(&mut self) -> SwrstS1ExtUsbW<'_, SwrstS1ExtSpec> {
        SwrstS1ExtUsbW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn swrst_s1_ext_mix2(&mut self) -> SwrstS1ExtMix2W<'_, SwrstS1ExtSpec> {
        SwrstS1ExtMix2W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn swrst_s1_ext_audio(&mut self) -> SwrstS1ExtAudioW<'_, SwrstS1ExtSpec> {
        SwrstS1ExtAudioW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn swrst_s1_ext_sdh(&mut self) -> SwrstS1ExtSdhW<'_, SwrstS1ExtSpec> {
        SwrstS1ExtSdhW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn swrst_s1_ext_emac(&mut self) -> SwrstS1ExtEmacW<'_, SwrstS1ExtSpec> {
        SwrstS1ExtEmacW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn swrst_s1_ext_dma2(&mut self) -> SwrstS1ExtDma2W<'_, SwrstS1ExtSpec> {
        SwrstS1ExtDma2W::new(self, 24)
    }
}
#[doc = "Software Reset Config 0 (EXT)\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_s1_ext::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_s1_ext::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwrstS1ExtSpec;
impl crate::RegisterSpec for SwrstS1ExtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swrst_s1_ext::R`](R) reader structure"]
impl crate::Readable for SwrstS1ExtSpec {}
#[doc = "`write(|w| ..)` method takes [`swrst_s1_ext::W`](W) writer structure"]
impl crate::Writable for SwrstS1ExtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWRST_S1_EXT to value 0"]
impl crate::Resettable for SwrstS1ExtSpec {}
