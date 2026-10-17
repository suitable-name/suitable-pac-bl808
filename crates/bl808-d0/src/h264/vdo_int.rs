#[doc = "Register `VDO_INT` reader"]
pub type R = crate::R<VdoIntSpec>;
#[doc = "Field `frm_done_int` reader - Main-stream frame done interrupt"]
pub type FrmDoneIntR = crate::BitReader;
#[doc = "Field `s_frm_done_int` reader - Secondary-stream frame done interrupt"]
pub type SFrmDoneIntR = crate::BitReader;
#[doc = "Field `seq_done_int` reader - Main-stream sequence done interrupt"]
pub type SeqDoneIntR = crate::BitReader;
#[doc = "Field `s_seq_done_int` reader - Secondary-stream sequence done interrupt"]
pub type SSeqDoneIntR = crate::BitReader;
#[doc = "Field `bs_full_int` reader - Main-stream bitstream buffer full interrupt"]
pub type BsFullIntR = crate::BitReader;
#[doc = "Field `s_bs_full_int` reader - Secondary-stream bitstream buffer full interrupt"]
pub type SBsFullIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Main-stream frame done interrupt"]
    #[inline(always)]
    pub fn frm_done_int(&self) -> FrmDoneIntR {
        FrmDoneIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Secondary-stream frame done interrupt"]
    #[inline(always)]
    pub fn s_frm_done_int(&self) -> SFrmDoneIntR {
        SFrmDoneIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Main-stream sequence done interrupt"]
    #[inline(always)]
    pub fn seq_done_int(&self) -> SeqDoneIntR {
        SeqDoneIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Secondary-stream sequence done interrupt"]
    #[inline(always)]
    pub fn s_seq_done_int(&self) -> SSeqDoneIntR {
        SSeqDoneIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Main-stream bitstream buffer full interrupt"]
    #[inline(always)]
    pub fn bs_full_int(&self) -> BsFullIntR {
        BsFullIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Secondary-stream bitstream buffer full interrupt"]
    #[inline(always)]
    pub fn s_bs_full_int(&self) -> SBsFullIntR {
        SBsFullIntR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Video encoder interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_int::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoIntSpec;
impl crate::RegisterSpec for VdoIntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdo_int::R`](R) reader structure"]
impl crate::Readable for VdoIntSpec {}
#[doc = "`reset()` method sets VDO_INT to value 0"]
impl crate::Resettable for VdoIntSpec {}
