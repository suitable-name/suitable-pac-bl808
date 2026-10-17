#[doc = "Register `SF_IF2_CTRL_0` reader"]
pub type R = crate::R<SfIf2Ctrl0Spec>;
#[doc = "Register `SF_IF2_CTRL_0` writer"]
pub type W = crate::W<SfIf2Ctrl0Spec>;
#[doc = "Field `sf_clk_sf_if2_rx_inv_sel` reader - "]
pub type SfClkSfIf2RxInvSelR = crate::BitReader;
#[doc = "Field `sf_clk_sf_if2_rx_inv_sel` writer - "]
pub type SfClkSfIf2RxInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if2_read_dly_n` reader - "]
pub type SfIf2ReadDlyNR = crate::FieldReader;
#[doc = "Field `sf_if2_read_dly_n` writer - "]
pub type SfIf2ReadDlyNW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_if2_read_dly_en` reader - "]
pub type SfIf2ReadDlyEnR = crate::BitReader;
#[doc = "Field `sf_if2_read_dly_en` writer - "]
pub type SfIf2ReadDlyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if2_int` reader - "]
pub type SfIf2IntR = crate::BitReader;
#[doc = "Field `sf_if2_int_clr` reader - "]
pub type SfIf2IntClrR = crate::BitReader;
#[doc = "Field `sf_if2_int_clr` writer - "]
pub type SfIf2IntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if2_int_set` reader - "]
pub type SfIf2IntSetR = crate::BitReader;
#[doc = "Field `sf_if2_int_set` writer - "]
pub type SfIf2IntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if2_replace_sf1` reader - "]
pub type SfIf2ReplaceSf1R = crate::BitReader;
#[doc = "Field `sf_if2_replace_sf1` writer - "]
pub type SfIf2ReplaceSf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if2_replace_sf2` reader - "]
pub type SfIf2ReplaceSf2R = crate::BitReader;
#[doc = "Field `sf_if2_replace_sf2` writer - "]
pub type SfIf2ReplaceSf2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if2_replace_sf3` reader - "]
pub type SfIf2ReplaceSf3R = crate::BitReader;
#[doc = "Field `sf_if2_replace_sf3` writer - "]
pub type SfIf2ReplaceSf3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if2_pad_sel` reader - "]
pub type SfIf2PadSelR = crate::FieldReader;
#[doc = "Field `sf_if2_pad_sel` writer - "]
pub type SfIf2PadSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf_if2_bk_swap` reader - "]
pub type SfIf2BkSwapR = crate::BitReader;
#[doc = "Field `sf_if2_bk_swap` writer - "]
pub type SfIf2BkSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if2_bk2_mode` reader - "]
pub type SfIf2Bk2ModeR = crate::BitReader;
#[doc = "Field `sf_if2_bk2_mode` writer - "]
pub type SfIf2Bk2ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if2_bk2_en` reader - "]
pub type SfIf2Bk2EnR = crate::BitReader;
#[doc = "Field `sf_if2_bk2_en` writer - "]
pub type SfIf2Bk2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if2_bk_sel` reader - "]
pub type SfIf2BkSelR = crate::BitReader;
#[doc = "Field `sf_if2_bk_sel` writer - "]
pub type SfIf2BkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_clk_sf_if2_rx_inv_sel(&self) -> SfClkSfIf2RxInvSelR {
        SfClkSfIf2RxInvSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sf_if2_read_dly_n(&self) -> SfIf2ReadDlyNR {
        SfIf2ReadDlyNR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sf_if2_read_dly_en(&self) -> SfIf2ReadDlyEnR {
        SfIf2ReadDlyEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sf_if2_int(&self) -> SfIf2IntR {
        SfIf2IntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sf_if2_int_clr(&self) -> SfIf2IntClrR {
        SfIf2IntClrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sf_if2_int_set(&self) -> SfIf2IntSetR {
        SfIf2IntSetR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_if2_replace_sf1(&self) -> SfIf2ReplaceSf1R {
        SfIf2ReplaceSf1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if2_replace_sf2(&self) -> SfIf2ReplaceSf2R {
        SfIf2ReplaceSf2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if2_replace_sf3(&self) -> SfIf2ReplaceSf3R {
        SfIf2ReplaceSf3R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sf_if2_pad_sel(&self) -> SfIf2PadSelR {
        SfIf2PadSelR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sf_if2_bk_swap(&self) -> SfIf2BkSwapR {
        SfIf2BkSwapR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_if2_bk2_mode(&self) -> SfIf2Bk2ModeR {
        SfIf2Bk2ModeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_if2_bk2_en(&self) -> SfIf2Bk2EnR {
        SfIf2Bk2EnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if2_bk_sel(&self) -> SfIf2BkSelR {
        SfIf2BkSelR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_clk_sf_if2_rx_inv_sel(&mut self) -> SfClkSfIf2RxInvSelW<'_, SfIf2Ctrl0Spec> {
        SfClkSfIf2RxInvSelW::new(self, 2)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sf_if2_read_dly_n(&mut self) -> SfIf2ReadDlyNW<'_, SfIf2Ctrl0Spec> {
        SfIf2ReadDlyNW::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sf_if2_read_dly_en(&mut self) -> SfIf2ReadDlyEnW<'_, SfIf2Ctrl0Spec> {
        SfIf2ReadDlyEnW::new(self, 11)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sf_if2_int_clr(&mut self) -> SfIf2IntClrW<'_, SfIf2Ctrl0Spec> {
        SfIf2IntClrW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sf_if2_int_set(&mut self) -> SfIf2IntSetW<'_, SfIf2Ctrl0Spec> {
        SfIf2IntSetW::new(self, 18)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_if2_replace_sf1(&mut self) -> SfIf2ReplaceSf1W<'_, SfIf2Ctrl0Spec> {
        SfIf2ReplaceSf1W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if2_replace_sf2(&mut self) -> SfIf2ReplaceSf2W<'_, SfIf2Ctrl0Spec> {
        SfIf2ReplaceSf2W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if2_replace_sf3(&mut self) -> SfIf2ReplaceSf3W<'_, SfIf2Ctrl0Spec> {
        SfIf2ReplaceSf3W::new(self, 25)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sf_if2_pad_sel(&mut self) -> SfIf2PadSelW<'_, SfIf2Ctrl0Spec> {
        SfIf2PadSelW::new(self, 26)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sf_if2_bk_swap(&mut self) -> SfIf2BkSwapW<'_, SfIf2Ctrl0Spec> {
        SfIf2BkSwapW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_if2_bk2_mode(&mut self) -> SfIf2Bk2ModeW<'_, SfIf2Ctrl0Spec> {
        SfIf2Bk2ModeW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_if2_bk2_en(&mut self) -> SfIf2Bk2EnW<'_, SfIf2Ctrl0Spec> {
        SfIf2Bk2EnW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if2_bk_sel(&mut self) -> SfIf2BkSelW<'_, SfIf2Ctrl0Spec> {
        SfIf2BkSelW::new(self, 31)
    }
}
#[doc = "Serial Flash 2 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if2_ctrl_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if2_ctrl_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIf2Ctrl0Spec;
impl crate::RegisterSpec for SfIf2Ctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if2_ctrl_0::R`](R) reader structure"]
impl crate::Readable for SfIf2Ctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_if2_ctrl_0::W`](W) writer structure"]
impl crate::Writable for SfIf2Ctrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_IF2_CTRL_0 to value 0"]
impl crate::Resettable for SfIf2Ctrl0Spec {}
