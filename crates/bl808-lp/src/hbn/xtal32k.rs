#[doc = "Register `XTAL32K` reader"]
pub type R = crate::R<Xtal32kSpec>;
#[doc = "Register `XTAL32K` writer"]
pub type W = crate::W<Xtal32kSpec>;
#[doc = "Field `xtal32k_ext_sel` reader - "]
pub type Xtal32kExtSelR = crate::BitReader;
#[doc = "Field `xtal32k_ext_sel` writer - "]
pub type Xtal32kExtSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal32k_amp_ctrl` reader - "]
pub type Xtal32kAmpCtrlR = crate::FieldReader;
#[doc = "Field `xtal32k_amp_ctrl` writer - "]
pub type Xtal32kAmpCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `xtal32k_reg` reader - "]
pub type Xtal32kRegR = crate::FieldReader;
#[doc = "Field `xtal32k_reg` writer - "]
pub type Xtal32kRegW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `xtal32k_outbuf_stre` reader - "]
pub type Xtal32kOutbufStreR = crate::BitReader;
#[doc = "Field `xtal32k_outbuf_stre` writer - "]
pub type Xtal32kOutbufStreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal32k_otf_short` reader - "]
pub type Xtal32kOtfShortR = crate::BitReader;
#[doc = "Field `xtal32k_otf_short` writer - "]
pub type Xtal32kOtfShortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal32k_inv_stre` reader - "]
pub type Xtal32kInvStreR = crate::FieldReader;
#[doc = "Field `xtal32k_inv_stre` writer - "]
pub type Xtal32kInvStreW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `xtal32k_capbank` reader - "]
pub type Xtal32kCapbankR = crate::FieldReader;
#[doc = "Field `xtal32k_capbank` writer - "]
pub type Xtal32kCapbankW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `xtal32k_ac_cap_short` reader - "]
pub type Xtal32kAcCapShortR = crate::BitReader;
#[doc = "Field `xtal32k_ac_cap_short` writer - "]
pub type Xtal32kAcCapShortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_xtal32k_buf` reader - "]
pub type PuXtal32kBufR = crate::BitReader;
#[doc = "Field `pu_xtal32k_buf` writer - "]
pub type PuXtal32kBufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_xtal32k` reader - "]
pub type PuXtal32kR = crate::BitReader;
#[doc = "Field `pu_xtal32k` writer - "]
pub type PuXtal32kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal32k_hiz_en` reader - "]
pub type Xtal32kHizEnR = crate::BitReader;
#[doc = "Field `xtal32k_hiz_en` writer - "]
pub type Xtal32kHizEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_xtal32k` reader - "]
pub type DtenXtal32kR = crate::BitReader;
#[doc = "Field `dten_xtal32k` writer - "]
pub type DtenXtal32kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_xtal32k` reader - "]
pub type TenXtal32kR = crate::BitReader;
#[doc = "Field `ten_xtal32k` writer - "]
pub type TenXtal32kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `f32k_sel_rtc` reader - "]
pub type F32kSelRtcR = crate::BitReader;
#[doc = "Field `f32k_sel_rtc` writer - "]
pub type F32kSelRtcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn xtal32k_ext_sel(&self) -> Xtal32kExtSelR {
        Xtal32kExtSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn xtal32k_amp_ctrl(&self) -> Xtal32kAmpCtrlR {
        Xtal32kAmpCtrlR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn xtal32k_reg(&self) -> Xtal32kRegR {
        Xtal32kRegR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn xtal32k_outbuf_stre(&self) -> Xtal32kOutbufStreR {
        Xtal32kOutbufStreR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn xtal32k_otf_short(&self) -> Xtal32kOtfShortR {
        Xtal32kOtfShortR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn xtal32k_inv_stre(&self) -> Xtal32kInvStreR {
        Xtal32kInvStreR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:16"]
    #[inline(always)]
    pub fn xtal32k_capbank(&self) -> Xtal32kCapbankR {
        Xtal32kCapbankR::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn xtal32k_ac_cap_short(&self) -> Xtal32kAcCapShortR {
        Xtal32kAcCapShortR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_xtal32k_buf(&self) -> PuXtal32kBufR {
        PuXtal32kBufR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_xtal32k(&self) -> PuXtal32kR {
        PuXtal32kR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn xtal32k_hiz_en(&self) -> Xtal32kHizEnR {
        Xtal32kHizEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dten_xtal32k(&self) -> DtenXtal32kR {
        DtenXtal32kR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ten_xtal32k(&self) -> TenXtal32kR {
        TenXtal32kR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn f32k_sel_rtc(&self) -> F32kSelRtcR {
        F32kSelRtcR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn xtal32k_ext_sel(&mut self) -> Xtal32kExtSelW<'_, Xtal32kSpec> {
        Xtal32kExtSelW::new(self, 2)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn xtal32k_amp_ctrl(&mut self) -> Xtal32kAmpCtrlW<'_, Xtal32kSpec> {
        Xtal32kAmpCtrlW::new(self, 3)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn xtal32k_reg(&mut self) -> Xtal32kRegW<'_, Xtal32kSpec> {
        Xtal32kRegW::new(self, 5)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn xtal32k_outbuf_stre(&mut self) -> Xtal32kOutbufStreW<'_, Xtal32kSpec> {
        Xtal32kOutbufStreW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn xtal32k_otf_short(&mut self) -> Xtal32kOtfShortW<'_, Xtal32kSpec> {
        Xtal32kOtfShortW::new(self, 8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn xtal32k_inv_stre(&mut self) -> Xtal32kInvStreW<'_, Xtal32kSpec> {
        Xtal32kInvStreW::new(self, 9)
    }
    #[doc = "Bits 11:16"]
    #[inline(always)]
    pub fn xtal32k_capbank(&mut self) -> Xtal32kCapbankW<'_, Xtal32kSpec> {
        Xtal32kCapbankW::new(self, 11)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn xtal32k_ac_cap_short(&mut self) -> Xtal32kAcCapShortW<'_, Xtal32kSpec> {
        Xtal32kAcCapShortW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_xtal32k_buf(&mut self) -> PuXtal32kBufW<'_, Xtal32kSpec> {
        PuXtal32kBufW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_xtal32k(&mut self) -> PuXtal32kW<'_, Xtal32kSpec> {
        PuXtal32kW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn xtal32k_hiz_en(&mut self) -> Xtal32kHizEnW<'_, Xtal32kSpec> {
        Xtal32kHizEnW::new(self, 20)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dten_xtal32k(&mut self) -> DtenXtal32kW<'_, Xtal32kSpec> {
        DtenXtal32kW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ten_xtal32k(&mut self) -> TenXtal32kW<'_, Xtal32kSpec> {
        TenXtal32kW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn f32k_sel_rtc(&mut self) -> F32kSelRtcW<'_, Xtal32kSpec> {
        F32kSelRtcW::new(self, 24)
    }
}
#[doc = "XTAL32K External Oscillator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal32k::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal32k::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xtal32kSpec;
impl crate::RegisterSpec for Xtal32kSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtal32k::R`](R) reader structure"]
impl crate::Readable for Xtal32kSpec {}
#[doc = "`write(|w| ..)` method takes [`xtal32k::W`](W) writer structure"]
impl crate::Writable for Xtal32kSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTAL32K to value 0x0015_0228"]
impl crate::Resettable for Xtal32kSpec {
    const RESET_VALUE: u32 = 0x0015_0228;
}
