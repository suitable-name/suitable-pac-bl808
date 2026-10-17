#[doc = "Register `DEV_ISG2` reader"]
pub type R = crate::R<DevIsg2Spec>;
#[doc = "Field `USBRST_INT` reader - USB reset interrupt status"]
pub type UsbrstIntR = crate::BitReader;
#[doc = "Field `SUSP_INT` reader - Suspend interrupt status"]
pub type SuspIntR = crate::BitReader;
#[doc = "Field `RESM_INT` reader - Resume interrupt status"]
pub type ResmIntR = crate::BitReader;
#[doc = "Field `ISO_SEQ_ERR_INT` reader - Isochronous Sequence Error"]
pub type IsoSeqErrIntR = crate::BitReader;
#[doc = "Field `ISO_SEQ_ABORT_INT` reader - Isochronous Sequence Abort"]
pub type IsoSeqAbortIntR = crate::BitReader;
#[doc = "Field `TX0BYTE_INT` reader - TX 0 byte interrupt"]
pub type Tx0byteIntR = crate::BitReader;
#[doc = "Field `RX0BYTE_INT` reader - RX 0 byte interrupt"]
pub type Rx0byteIntR = crate::BitReader;
#[doc = "Field `DMA_CMPLT_HOV` reader - DMA completion (Host Overwrite)"]
pub type DmaCmpltHovR = crate::BitReader;
#[doc = "Field `DMA_ERROR_HOV` reader - DMA error (Host Overwrite)"]
pub type DmaErrorHovR = crate::BitReader;
#[doc = "Field `DEV_IDLE_HOV` reader - Device Idle (Host Overwrite)"]
pub type DevIdleHovR = crate::BitReader;
#[doc = "Field `DEV_WAKEUP_BYVBUS` reader - Device Wakeup by VBUS"]
pub type DevWakeupByvbusR = crate::BitReader;
#[doc = "Field `F8_OUT_INT` reader - "]
pub type F8OutIntR = crate::BitReader;
#[doc = "Field `F8_SPK_INT` reader - "]
pub type F8SpkIntR = crate::BitReader;
#[doc = "Field `F9_OUT_INT` reader - "]
pub type F9OutIntR = crate::BitReader;
#[doc = "Field `F9_SPK_INT` reader - "]
pub type F9SpkIntR = crate::BitReader;
#[doc = "Field `F10_OUT_INT` reader - "]
pub type F10OutIntR = crate::BitReader;
#[doc = "Field `F10_SPK_INT` reader - "]
pub type F10SpkIntR = crate::BitReader;
#[doc = "Field `F11_OUT_INT` reader - "]
pub type F11OutIntR = crate::BitReader;
#[doc = "Field `F11_SPK_INT` reader - "]
pub type F11SpkIntR = crate::BitReader;
#[doc = "Field `F12_OUT_INT` reader - "]
pub type F12OutIntR = crate::BitReader;
#[doc = "Field `F12_SPK_INT` reader - "]
pub type F12SpkIntR = crate::BitReader;
#[doc = "Field `F13_OUT_INT` reader - "]
pub type F13OutIntR = crate::BitReader;
#[doc = "Field `F13_SPK_INT` reader - "]
pub type F13SpkIntR = crate::BitReader;
#[doc = "Field `F14_OUT_INT` reader - "]
pub type F14OutIntR = crate::BitReader;
#[doc = "Field `F14_SPK_INT` reader - "]
pub type F14SpkIntR = crate::BitReader;
#[doc = "Field `F15_OUT_INT` reader - "]
pub type F15OutIntR = crate::BitReader;
#[doc = "Field `F15_SPK_INT` reader - "]
pub type F15SpkIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - USB reset interrupt status"]
    #[inline(always)]
    pub fn usbrst_int(&self) -> UsbrstIntR {
        UsbrstIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Suspend interrupt status"]
    #[inline(always)]
    pub fn susp_int(&self) -> SuspIntR {
        SuspIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Resume interrupt status"]
    #[inline(always)]
    pub fn resm_int(&self) -> ResmIntR {
        ResmIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Isochronous Sequence Error"]
    #[inline(always)]
    pub fn iso_seq_err_int(&self) -> IsoSeqErrIntR {
        IsoSeqErrIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Isochronous Sequence Abort"]
    #[inline(always)]
    pub fn iso_seq_abort_int(&self) -> IsoSeqAbortIntR {
        IsoSeqAbortIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX 0 byte interrupt"]
    #[inline(always)]
    pub fn tx0byte_int(&self) -> Tx0byteIntR {
        Tx0byteIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX 0 byte interrupt"]
    #[inline(always)]
    pub fn rx0byte_int(&self) -> Rx0byteIntR {
        Rx0byteIntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA completion (Host Overwrite)"]
    #[inline(always)]
    pub fn dma_cmplt_hov(&self) -> DmaCmpltHovR {
        DmaCmpltHovR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA error (Host Overwrite)"]
    #[inline(always)]
    pub fn dma_error_hov(&self) -> DmaErrorHovR {
        DmaErrorHovR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Device Idle (Host Overwrite)"]
    #[inline(always)]
    pub fn dev_idle_hov(&self) -> DevIdleHovR {
        DevIdleHovR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Device Wakeup by VBUS"]
    #[inline(always)]
    pub fn dev_wakeup_byvbus(&self) -> DevWakeupByvbusR {
        DevWakeupByvbusR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn f8_out_int(&self) -> F8OutIntR {
        F8OutIntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn f8_spk_int(&self) -> F8SpkIntR {
        F8SpkIntR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn f9_out_int(&self) -> F9OutIntR {
        F9OutIntR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn f9_spk_int(&self) -> F9SpkIntR {
        F9SpkIntR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn f10_out_int(&self) -> F10OutIntR {
        F10OutIntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn f10_spk_int(&self) -> F10SpkIntR {
        F10SpkIntR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn f11_out_int(&self) -> F11OutIntR {
        F11OutIntR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn f11_spk_int(&self) -> F11SpkIntR {
        F11SpkIntR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn f12_out_int(&self) -> F12OutIntR {
        F12OutIntR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn f12_spk_int(&self) -> F12SpkIntR {
        F12SpkIntR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn f13_out_int(&self) -> F13OutIntR {
        F13OutIntR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn f13_spk_int(&self) -> F13SpkIntR {
        F13SpkIntR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn f14_out_int(&self) -> F14OutIntR {
        F14OutIntR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn f14_spk_int(&self) -> F14SpkIntR {
        F14SpkIntR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn f15_out_int(&self) -> F15OutIntR {
        F15OutIntR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn f15_spk_int(&self) -> F15SpkIntR {
        F15SpkIntR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Device Interrupt Status Group 2 (Events/DMA/FIFO 8-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_isg2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevIsg2Spec;
impl crate::RegisterSpec for DevIsg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_isg2::R`](R) reader structure"]
impl crate::Readable for DevIsg2Spec {}
#[doc = "`reset()` method sets DEV_ISG2 to value 0"]
impl crate::Resettable for DevIsg2Spec {}
