#[doc = "Register `CGEN_S1_EXT` reader"]
pub type R = crate::R<CgenS1ExtSpec>;
#[doc = "Register `CGEN_S1_EXT` writer"]
pub type W = crate::W<CgenS1ExtSpec>;
#[doc = "Field `cgen_s0` reader - "]
pub type CgenS0R = crate::BitReader;
#[doc = "Field `cgen_s0` writer - "]
pub type CgenS0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s2_wifi` reader - "]
pub type CgenS2WifiR = crate::BitReader;
#[doc = "Field `cgen_s2_wifi` writer - "]
pub type CgenS2WifiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s3_bt_ble2` reader - "]
pub type CgenS3BtBle2R = crate::BitReader;
#[doc = "Field `cgen_s3_bt_ble2` writer - "]
pub type CgenS3BtBle2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s3_m1542` reader - "]
pub type CgenS3M1542R = crate::BitReader;
#[doc = "Field `cgen_s3_m1542` writer - "]
pub type CgenS3M1542W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_ext_emi_misc` reader - "]
pub type CgenS1ExtEmiMiscR = crate::BitReader;
#[doc = "Field `cgen_s1_ext_emi_misc` writer - "]
pub type CgenS1ExtEmiMiscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_ext_psram0_ctrl` reader - "]
pub type CgenS1ExtPsram0CtrlR = crate::BitReader;
#[doc = "Field `cgen_s1_ext_psram0_ctrl` writer - "]
pub type CgenS1ExtPsram0CtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_ext_psram_ctrl` reader - "]
pub type CgenS1ExtPsramCtrlR = crate::BitReader;
#[doc = "Field `cgen_s1_ext_psram_ctrl` writer - "]
pub type CgenS1ExtPsramCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_ext_usb` reader - "]
pub type CgenS1ExtUsbR = crate::BitReader;
#[doc = "Field `cgen_s1_ext_usb` writer - "]
pub type CgenS1ExtUsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_ext_mix2` reader - "]
pub type CgenS1ExtMix2R = crate::BitReader;
#[doc = "Field `cgen_s1_ext_mix2` writer - "]
pub type CgenS1ExtMix2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_ext_audio` reader - "]
pub type CgenS1ExtAudioR = crate::BitReader;
#[doc = "Field `cgen_s1_ext_audio` writer - "]
pub type CgenS1ExtAudioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_ext_sdh` reader - "]
pub type CgenS1ExtSdhR = crate::BitReader;
#[doc = "Field `cgen_s1_ext_sdh` writer - "]
pub type CgenS1ExtSdhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_ext_emac` reader - "]
pub type CgenS1ExtEmacR = crate::BitReader;
#[doc = "Field `cgen_s1_ext_emac` writer - "]
pub type CgenS1ExtEmacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_s1_ext_dma2` reader - "]
pub type CgenS1ExtDma2R = crate::BitReader;
#[doc = "Field `cgen_s1_ext_dma2` writer - "]
pub type CgenS1ExtDma2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cgen_s0(&self) -> CgenS0R {
        CgenS0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cgen_s2_wifi(&self) -> CgenS2WifiR {
        CgenS2WifiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cgen_s3_bt_ble2(&self) -> CgenS3BtBle2R {
        CgenS3BtBle2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cgen_s3_m1542(&self) -> CgenS3M1542R {
        CgenS3M1542R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cgen_s1_ext_emi_misc(&self) -> CgenS1ExtEmiMiscR {
        CgenS1ExtEmiMiscR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cgen_s1_ext_psram0_ctrl(&self) -> CgenS1ExtPsram0CtrlR {
        CgenS1ExtPsram0CtrlR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cgen_s1_ext_psram_ctrl(&self) -> CgenS1ExtPsramCtrlR {
        CgenS1ExtPsramCtrlR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cgen_s1_ext_usb(&self) -> CgenS1ExtUsbR {
        CgenS1ExtUsbR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cgen_s1_ext_mix2(&self) -> CgenS1ExtMix2R {
        CgenS1ExtMix2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cgen_s1_ext_audio(&self) -> CgenS1ExtAudioR {
        CgenS1ExtAudioR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cgen_s1_ext_sdh(&self) -> CgenS1ExtSdhR {
        CgenS1ExtSdhR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cgen_s1_ext_emac(&self) -> CgenS1ExtEmacR {
        CgenS1ExtEmacR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cgen_s1_ext_dma2(&self) -> CgenS1ExtDma2R {
        CgenS1ExtDma2R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cgen_s0(&mut self) -> CgenS0W<'_, CgenS1ExtSpec> {
        CgenS0W::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cgen_s2_wifi(&mut self) -> CgenS2WifiW<'_, CgenS1ExtSpec> {
        CgenS2WifiW::new(self, 4)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cgen_s3_bt_ble2(&mut self) -> CgenS3BtBle2W<'_, CgenS1ExtSpec> {
        CgenS3BtBle2W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cgen_s3_m1542(&mut self) -> CgenS3M1542W<'_, CgenS1ExtSpec> {
        CgenS3M1542W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cgen_s1_ext_emi_misc(&mut self) -> CgenS1ExtEmiMiscW<'_, CgenS1ExtSpec> {
        CgenS1ExtEmiMiscW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cgen_s1_ext_psram0_ctrl(&mut self) -> CgenS1ExtPsram0CtrlW<'_, CgenS1ExtSpec> {
        CgenS1ExtPsram0CtrlW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cgen_s1_ext_psram_ctrl(&mut self) -> CgenS1ExtPsramCtrlW<'_, CgenS1ExtSpec> {
        CgenS1ExtPsramCtrlW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cgen_s1_ext_usb(&mut self) -> CgenS1ExtUsbW<'_, CgenS1ExtSpec> {
        CgenS1ExtUsbW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cgen_s1_ext_mix2(&mut self) -> CgenS1ExtMix2W<'_, CgenS1ExtSpec> {
        CgenS1ExtMix2W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cgen_s1_ext_audio(&mut self) -> CgenS1ExtAudioW<'_, CgenS1ExtSpec> {
        CgenS1ExtAudioW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cgen_s1_ext_sdh(&mut self) -> CgenS1ExtSdhW<'_, CgenS1ExtSpec> {
        CgenS1ExtSdhW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cgen_s1_ext_emac(&mut self) -> CgenS1ExtEmacW<'_, CgenS1ExtSpec> {
        CgenS1ExtEmacW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cgen_s1_ext_dma2(&mut self) -> CgenS1ExtDma2W<'_, CgenS1ExtSpec> {
        CgenS1ExtDma2W::new(self, 24)
    }
}
#[doc = "Clock Gate Config 2 (S3 + S1_EXT Peripherals)\n\nYou can [`read`](crate::Reg::read) this register and get [`cgen_s1_ext::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgen_s1_ext::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CgenS1ExtSpec;
impl crate::RegisterSpec for CgenS1ExtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgen_s1_ext::R`](R) reader structure"]
impl crate::Readable for CgenS1ExtSpec {}
#[doc = "`write(|w| ..)` method takes [`cgen_s1_ext::W`](W) writer structure"]
impl crate::Writable for CgenS1ExtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CGEN_S1_EXT to value 0"]
impl crate::Resettable for CgenS1ExtSpec {}
