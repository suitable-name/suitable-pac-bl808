#[doc = "Register `SF_CTRL_2` reader"]
pub type R = crate::R<SfCtrl2Spec>;
#[doc = "Register `SF_CTRL_2` writer"]
pub type W = crate::W<SfCtrl2Spec>;
#[doc = "Field `sf_if_pad_sel` reader - "]
pub type SfIfPadSelR = crate::FieldReader;
#[doc = "Field `sf_if_pad_sel` writer - "]
pub type SfIfPadSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf_if_pad_sel_lock` reader - "]
pub type SfIfPadSelLockR = crate::BitReader;
#[doc = "Field `sf_if_pad_sel_lock` writer - "]
pub type SfIfPadSelLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_dtr_en` reader - "]
pub type SfIfDtrEnR = crate::BitReader;
#[doc = "Field `sf_if_dtr_en` writer - "]
pub type SfIfDtrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_dqs_en` reader - "]
pub type SfIfDqsEnR = crate::BitReader;
#[doc = "Field `sf_if_dqs_en` writer - "]
pub type SfIfDqsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_trig_wr_prot` reader - "]
pub type SfIfTrigWrProtR = crate::BitReader;
#[doc = "Field `sf_if_trig_wr_prot` writer - "]
pub type SfIfTrigWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_id_offset_lock` reader - "]
pub type SfIdOffsetLockR = crate::BitReader;
#[doc = "Field `sf_id_offset_lock` writer - "]
pub type SfIdOffsetLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_ahb2sif_remap_lock` reader - "]
pub type SfAhb2sifRemapLockR = crate::BitReader;
#[doc = "Field `sf_ahb2sif_remap_lock` writer - "]
pub type SfAhb2sifRemapLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_ahb2sif_remap` reader - "]
pub type SfAhb2sifRemapR = crate::FieldReader;
#[doc = "Field `sf_ahb2sif_remap` writer - "]
pub type SfAhb2sifRemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf_if_bk_swap` reader - "]
pub type SfIfBkSwapR = crate::BitReader;
#[doc = "Field `sf_if_bk_swap` writer - "]
pub type SfIfBkSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_bk2_mode` reader - "]
pub type SfIfBk2ModeR = crate::BitReader;
#[doc = "Field `sf_if_bk2_mode` writer - "]
pub type SfIfBk2ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_bk2_en` reader - "]
pub type SfIfBk2EnR = crate::BitReader;
#[doc = "Field `sf_if_bk2_en` writer - "]
pub type SfIfBk2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_0_bk_sel` reader - "]
pub type SfIf0BkSelR = crate::BitReader;
#[doc = "Field `sf_if_0_bk_sel` writer - "]
pub type SfIf0BkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf_if_pad_sel(&self) -> SfIfPadSelR {
        SfIfPadSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_if_pad_sel_lock(&self) -> SfIfPadSelLockR {
        SfIfPadSelLockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_if_dtr_en(&self) -> SfIfDtrEnR {
        SfIfDtrEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sf_if_dqs_en(&self) -> SfIfDqsEnR {
        SfIfDqsEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sf_if_trig_wr_prot(&self) -> SfIfTrigWrProtR {
        SfIfTrigWrProtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sf_id_offset_lock(&self) -> SfIdOffsetLockR {
        SfIdOffsetLockR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_ahb2sif_remap_lock(&self) -> SfAhb2sifRemapLockR {
        SfAhb2sifRemapLockR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sf_ahb2sif_remap(&self) -> SfAhb2sifRemapR {
        SfAhb2sifRemapR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sf_if_bk_swap(&self) -> SfIfBkSwapR {
        SfIfBkSwapR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_if_bk2_mode(&self) -> SfIfBk2ModeR {
        SfIfBk2ModeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_if_bk2_en(&self) -> SfIfBk2EnR {
        SfIfBk2EnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if_0_bk_sel(&self) -> SfIf0BkSelR {
        SfIf0BkSelR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf_if_pad_sel(&mut self) -> SfIfPadSelW<'_, SfCtrl2Spec> {
        SfIfPadSelW::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_if_pad_sel_lock(&mut self) -> SfIfPadSelLockW<'_, SfCtrl2Spec> {
        SfIfPadSelLockW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_if_dtr_en(&mut self) -> SfIfDtrEnW<'_, SfCtrl2Spec> {
        SfIfDtrEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sf_if_dqs_en(&mut self) -> SfIfDqsEnW<'_, SfCtrl2Spec> {
        SfIfDqsEnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sf_if_trig_wr_prot(&mut self) -> SfIfTrigWrProtW<'_, SfCtrl2Spec> {
        SfIfTrigWrProtW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sf_id_offset_lock(&mut self) -> SfIdOffsetLockW<'_, SfCtrl2Spec> {
        SfIdOffsetLockW::new(self, 7)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_ahb2sif_remap_lock(&mut self) -> SfAhb2sifRemapLockW<'_, SfCtrl2Spec> {
        SfAhb2sifRemapLockW::new(self, 25)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sf_ahb2sif_remap(&mut self) -> SfAhb2sifRemapW<'_, SfCtrl2Spec> {
        SfAhb2sifRemapW::new(self, 26)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sf_if_bk_swap(&mut self) -> SfIfBkSwapW<'_, SfCtrl2Spec> {
        SfIfBkSwapW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_if_bk2_mode(&mut self) -> SfIfBk2ModeW<'_, SfCtrl2Spec> {
        SfIfBk2ModeW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_if_bk2_en(&mut self) -> SfIfBk2EnW<'_, SfCtrl2Spec> {
        SfIfBk2EnW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if_0_bk_sel(&mut self) -> SfIf0BkSelW<'_, SfCtrl2Spec> {
        SfIf0BkSelW::new(self, 31)
    }
}
#[doc = "Serial Flash Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ctrl_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_ctrl_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfCtrl2Spec;
impl crate::RegisterSpec for SfCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_ctrl_2::R`](R) reader structure"]
impl crate::Readable for SfCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_ctrl_2::W`](W) writer structure"]
impl crate::Writable for SfCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_CTRL_2 to value 0"]
impl crate::Resettable for SfCtrl2Spec {}
