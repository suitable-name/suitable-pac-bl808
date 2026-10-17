#[doc = "Register `SD_PRESENT_STATE_1` reader"]
pub type R = crate::R<SdPresentState1Spec>;
#[doc = "Field `CMD_INHIBIT_CMD` reader - "]
pub type CmdInhibitCmdR = crate::BitReader;
#[doc = "Field `CMD_INHIBIT_DAT` reader - "]
pub type CmdInhibitDatR = crate::BitReader;
#[doc = "Field `_DAT_ACTIVE` reader - "]
pub type _DatActiveR = crate::BitReader;
#[doc = "Field `RETUNING_REQ` reader - "]
pub type RetuningReqR = crate::BitReader;
#[doc = "Field `TX_ACTIVE` reader - "]
pub type TxActiveR = crate::BitReader;
#[doc = "Field `RX_ACTIVE` reader - "]
pub type RxActiveR = crate::BitReader;
#[doc = "Field `BUFFER_WR_EN` reader - "]
pub type BufferWrEnR = crate::BitReader;
#[doc = "Field `BUFFER_RD_EN` reader - "]
pub type BufferRdEnR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmd_inhibit_cmd(&self) -> CmdInhibitCmdR {
        CmdInhibitCmdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cmd_inhibit_dat(&self) -> CmdInhibitDatR {
        CmdInhibitDatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn _dat_active(&self) -> _DatActiveR {
        _DatActiveR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn retuning_req(&self) -> RetuningReqR {
        RetuningReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_active(&self) -> TxActiveR {
        TxActiveR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rx_active(&self) -> RxActiveR {
        RxActiveR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn buffer_wr_en(&self) -> BufferWrEnR {
        BufferWrEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn buffer_rd_en(&self) -> BufferRdEnR {
        BufferRdEnR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Present State Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_present_state_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdPresentState1Spec;
impl crate::RegisterSpec for SdPresentState1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_present_state_1::R`](R) reader structure"]
impl crate::Readable for SdPresentState1Spec {}
#[doc = "`reset()` method sets SD_PRESENT_STATE_1 to value 0"]
impl crate::Resettable for SdPresentState1Spec {}
