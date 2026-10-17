#[doc = "Register `DEV_MISG2` reader"]
pub type R = crate::R<DevMisg2Spec>;
#[doc = "Field `MUSBRST_INT` reader - "]
pub type MusbrstIntR = crate::BitReader;
#[doc = "Field `MSUSP_INT` reader - "]
pub type MsuspIntR = crate::BitReader;
#[doc = "Field `MRESM_INT` reader - "]
pub type MresmIntR = crate::BitReader;
#[doc = "Field `MSEQ_ERR_INT` reader - "]
pub type MseqErrIntR = crate::BitReader;
#[doc = "Field `MSEQ_ABORT_INT` reader - "]
pub type MseqAbortIntR = crate::BitReader;
#[doc = "Field `MTX0BYTE_INT` reader - "]
pub type Mtx0byteIntR = crate::BitReader;
#[doc = "Field `MRX0BYTE_INT` reader - "]
pub type Mrx0byteIntR = crate::BitReader;
#[doc = "Field `MDMA_CMPLT_HOV` reader - "]
pub type MdmaCmpltHovR = crate::BitReader;
#[doc = "Field `MDMA_ERROR_HOV` reader - "]
pub type MdmaErrorHovR = crate::BitReader;
#[doc = "Field `MDEV_IDLE_HOV` reader - "]
pub type MdevIdleHovR = crate::BitReader;
#[doc = "Field `MDEV_WAKEUP_BYVBUS` reader - "]
pub type MdevWakeupByvbusR = crate::BitReader;
#[doc = "Field `MF8_OUT_INT` reader - "]
pub type Mf8OutIntR = crate::BitReader;
#[doc = "Field `MF8_SPK_INT` reader - "]
pub type Mf8SpkIntR = crate::BitReader;
#[doc = "Field `MF9_OUT_INT` reader - "]
pub type Mf9OutIntR = crate::BitReader;
#[doc = "Field `MF9_SPK_INT` reader - "]
pub type Mf9SpkIntR = crate::BitReader;
#[doc = "Field `MF10_OUT_INT` reader - "]
pub type Mf10OutIntR = crate::BitReader;
#[doc = "Field `MF10_SPK_INT` reader - "]
pub type Mf10SpkIntR = crate::BitReader;
#[doc = "Field `MF11_OUT_INT` reader - "]
pub type Mf11OutIntR = crate::BitReader;
#[doc = "Field `MF11_SPK_INT` reader - "]
pub type Mf11SpkIntR = crate::BitReader;
#[doc = "Field `MF12_OUT_INT` reader - "]
pub type Mf12OutIntR = crate::BitReader;
#[doc = "Field `MF12_SPK_INT` reader - "]
pub type Mf12SpkIntR = crate::BitReader;
#[doc = "Field `MF13_OUT_INT` reader - "]
pub type Mf13OutIntR = crate::BitReader;
#[doc = "Field `MF13_SPK_INT` reader - "]
pub type Mf13SpkIntR = crate::BitReader;
#[doc = "Field `MF14_OUT_INT` reader - "]
pub type Mf14OutIntR = crate::BitReader;
#[doc = "Field `MF14_SPK_INT` reader - "]
pub type Mf14SpkIntR = crate::BitReader;
#[doc = "Field `MF15_OUT_INT` reader - "]
pub type Mf15OutIntR = crate::BitReader;
#[doc = "Field `MF15_SPK_INT` reader - "]
pub type Mf15SpkIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn musbrst_int(&self) -> MusbrstIntR {
        MusbrstIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn msusp_int(&self) -> MsuspIntR {
        MsuspIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn mresm_int(&self) -> MresmIntR {
        MresmIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn mseq_err_int(&self) -> MseqErrIntR {
        MseqErrIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mseq_abort_int(&self) -> MseqAbortIntR {
        MseqAbortIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn mtx0byte_int(&self) -> Mtx0byteIntR {
        Mtx0byteIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn mrx0byte_int(&self) -> Mrx0byteIntR {
        Mrx0byteIntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn mdma_cmplt_hov(&self) -> MdmaCmpltHovR {
        MdmaCmpltHovR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn mdma_error_hov(&self) -> MdmaErrorHovR {
        MdmaErrorHovR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn mdev_idle_hov(&self) -> MdevIdleHovR {
        MdevIdleHovR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn mdev_wakeup_byvbus(&self) -> MdevWakeupByvbusR {
        MdevWakeupByvbusR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mf8_out_int(&self) -> Mf8OutIntR {
        Mf8OutIntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn mf8_spk_int(&self) -> Mf8SpkIntR {
        Mf8SpkIntR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn mf9_out_int(&self) -> Mf9OutIntR {
        Mf9OutIntR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn mf9_spk_int(&self) -> Mf9SpkIntR {
        Mf9SpkIntR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn mf10_out_int(&self) -> Mf10OutIntR {
        Mf10OutIntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn mf10_spk_int(&self) -> Mf10SpkIntR {
        Mf10SpkIntR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn mf11_out_int(&self) -> Mf11OutIntR {
        Mf11OutIntR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mf11_spk_int(&self) -> Mf11SpkIntR {
        Mf11SpkIntR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn mf12_out_int(&self) -> Mf12OutIntR {
        Mf12OutIntR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn mf12_spk_int(&self) -> Mf12SpkIntR {
        Mf12SpkIntR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn mf13_out_int(&self) -> Mf13OutIntR {
        Mf13OutIntR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn mf13_spk_int(&self) -> Mf13SpkIntR {
        Mf13SpkIntR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn mf14_out_int(&self) -> Mf14OutIntR {
        Mf14OutIntR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn mf14_spk_int(&self) -> Mf14SpkIntR {
        Mf14SpkIntR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn mf15_out_int(&self) -> Mf15OutIntR {
        Mf15OutIntR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn mf15_spk_int(&self) -> Mf15SpkIntR {
        Mf15SpkIntR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Device Masked Interrupt Status Group 2 (Events/DMA/FIFO 8-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_misg2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevMisg2Spec;
impl crate::RegisterSpec for DevMisg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_misg2::R`](R) reader structure"]
impl crate::Readable for DevMisg2Spec {}
#[doc = "`reset()` method sets DEV_MISG2 to value 0"]
impl crate::Resettable for DevMisg2Spec {}
