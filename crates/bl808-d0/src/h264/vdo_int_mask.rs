#[doc = "Register `VDO_INT_MASK` reader"]
pub type R = crate::R<VdoIntMaskSpec>;
#[doc = "Register `VDO_INT_MASK` writer"]
pub type W = crate::W<VdoIntMaskSpec>;
#[doc = "Field `frm_done_int_mask` reader - Mask main-stream frame done interrupt"]
pub type FrmDoneIntMaskR = crate::BitReader;
#[doc = "Field `frm_done_int_mask` writer - Mask main-stream frame done interrupt"]
pub type FrmDoneIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `s_frm_done_int_mask` reader - Mask secondary-stream frame done interrupt"]
pub type SFrmDoneIntMaskR = crate::BitReader;
#[doc = "Field `s_frm_done_int_mask` writer - Mask secondary-stream frame done interrupt"]
pub type SFrmDoneIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `seq_done_int_mask` reader - Mask main-stream sequence done interrupt"]
pub type SeqDoneIntMaskR = crate::BitReader;
#[doc = "Field `seq_done_int_mask` writer - Mask main-stream sequence done interrupt"]
pub type SeqDoneIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `s_seq_done_int_mask` reader - Mask secondary-stream sequence done interrupt"]
pub type SSeqDoneIntMaskR = crate::BitReader;
#[doc = "Field `s_seq_done_int_mask` writer - Mask secondary-stream sequence done interrupt"]
pub type SSeqDoneIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bs_full_int_mask` reader - Mask main-stream bitstream full interrupt"]
pub type BsFullIntMaskR = crate::BitReader;
#[doc = "Field `bs_full_int_mask` writer - Mask main-stream bitstream full interrupt"]
pub type BsFullIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `s_bs_full_int_mask` reader - Mask secondary-stream bitstream full interrupt"]
pub type SBsFullIntMaskR = crate::BitReader;
#[doc = "Field `s_bs_full_int_mask` writer - Mask secondary-stream bitstream full interrupt"]
pub type SBsFullIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask main-stream frame done interrupt"]
    #[inline(always)]
    pub fn frm_done_int_mask(&self) -> FrmDoneIntMaskR {
        FrmDoneIntMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask secondary-stream frame done interrupt"]
    #[inline(always)]
    pub fn s_frm_done_int_mask(&self) -> SFrmDoneIntMaskR {
        SFrmDoneIntMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask main-stream sequence done interrupt"]
    #[inline(always)]
    pub fn seq_done_int_mask(&self) -> SeqDoneIntMaskR {
        SeqDoneIntMaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask secondary-stream sequence done interrupt"]
    #[inline(always)]
    pub fn s_seq_done_int_mask(&self) -> SSeqDoneIntMaskR {
        SSeqDoneIntMaskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask main-stream bitstream full interrupt"]
    #[inline(always)]
    pub fn bs_full_int_mask(&self) -> BsFullIntMaskR {
        BsFullIntMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask secondary-stream bitstream full interrupt"]
    #[inline(always)]
    pub fn s_bs_full_int_mask(&self) -> SBsFullIntMaskR {
        SBsFullIntMaskR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask main-stream frame done interrupt"]
    #[inline(always)]
    pub fn frm_done_int_mask(&mut self) -> FrmDoneIntMaskW<'_, VdoIntMaskSpec> {
        FrmDoneIntMaskW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask secondary-stream frame done interrupt"]
    #[inline(always)]
    pub fn s_frm_done_int_mask(&mut self) -> SFrmDoneIntMaskW<'_, VdoIntMaskSpec> {
        SFrmDoneIntMaskW::new(self, 1)
    }
    #[doc = "Bit 4 - Mask main-stream sequence done interrupt"]
    #[inline(always)]
    pub fn seq_done_int_mask(&mut self) -> SeqDoneIntMaskW<'_, VdoIntMaskSpec> {
        SeqDoneIntMaskW::new(self, 4)
    }
    #[doc = "Bit 5 - Mask secondary-stream sequence done interrupt"]
    #[inline(always)]
    pub fn s_seq_done_int_mask(&mut self) -> SSeqDoneIntMaskW<'_, VdoIntMaskSpec> {
        SSeqDoneIntMaskW::new(self, 5)
    }
    #[doc = "Bit 8 - Mask main-stream bitstream full interrupt"]
    #[inline(always)]
    pub fn bs_full_int_mask(&mut self) -> BsFullIntMaskW<'_, VdoIntMaskSpec> {
        BsFullIntMaskW::new(self, 8)
    }
    #[doc = "Bit 9 - Mask secondary-stream bitstream full interrupt"]
    #[inline(always)]
    pub fn s_bs_full_int_mask(&mut self) -> SBsFullIntMaskW<'_, VdoIntMaskSpec> {
        SBsFullIntMaskW::new(self, 9)
    }
}
#[doc = "Video encoder interrupt mask register (1=masked)\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_int_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_int_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoIntMaskSpec;
impl crate::RegisterSpec for VdoIntMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdo_int_mask::R`](R) reader structure"]
impl crate::Readable for VdoIntMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`vdo_int_mask::W`](W) writer structure"]
impl crate::Writable for VdoIntMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDO_INT_MASK to value 0"]
impl crate::Resettable for VdoIntMaskSpec {}
