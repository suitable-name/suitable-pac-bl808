#[doc = "Register `VDO_INT_CLR` writer"]
pub type W = crate::W<VdoIntClrSpec>;
#[doc = "Field `frm_done_int_clr` writer - Clear main-stream frame done interrupt"]
pub type FrmDoneIntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `s_frm_done_int_clr` writer - Clear secondary-stream frame done interrupt"]
pub type SFrmDoneIntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `seq_done_int_clr` writer - Clear main-stream sequence done interrupt"]
pub type SeqDoneIntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `s_seq_done_int_clr` writer - Clear secondary-stream sequence done interrupt"]
pub type SSeqDoneIntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `bs_full_int_clr` writer - Clear main-stream bitstream full interrupt"]
pub type BsFullIntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `s_bs_full_int_clr` writer - Clear secondary-stream bitstream full interrupt"]
pub type SBsFullIntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear main-stream frame done interrupt"]
    #[inline(always)]
    pub fn frm_done_int_clr(&mut self) -> FrmDoneIntClrW<'_, VdoIntClrSpec> {
        FrmDoneIntClrW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear secondary-stream frame done interrupt"]
    #[inline(always)]
    pub fn s_frm_done_int_clr(&mut self) -> SFrmDoneIntClrW<'_, VdoIntClrSpec> {
        SFrmDoneIntClrW::new(self, 1)
    }
    #[doc = "Bit 4 - Clear main-stream sequence done interrupt"]
    #[inline(always)]
    pub fn seq_done_int_clr(&mut self) -> SeqDoneIntClrW<'_, VdoIntClrSpec> {
        SeqDoneIntClrW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear secondary-stream sequence done interrupt"]
    #[inline(always)]
    pub fn s_seq_done_int_clr(&mut self) -> SSeqDoneIntClrW<'_, VdoIntClrSpec> {
        SSeqDoneIntClrW::new(self, 5)
    }
    #[doc = "Bit 8 - Clear main-stream bitstream full interrupt"]
    #[inline(always)]
    pub fn bs_full_int_clr(&mut self) -> BsFullIntClrW<'_, VdoIntClrSpec> {
        BsFullIntClrW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear secondary-stream bitstream full interrupt"]
    #[inline(always)]
    pub fn s_bs_full_int_clr(&mut self) -> SBsFullIntClrW<'_, VdoIntClrSpec> {
        SBsFullIntClrW::new(self, 9)
    }
}
#[doc = "Video encoder interrupt clear register (write 1 to clear)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoIntClrSpec;
impl crate::RegisterSpec for VdoIntClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`vdo_int_clr::W`](W) writer structure"]
impl crate::Writable for VdoIntClrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0333;
}
#[doc = "`reset()` method sets VDO_INT_CLR to value 0"]
impl crate::Resettable for VdoIntClrSpec {}
