#[doc = "Register `DEV_MISG3` reader"]
pub type R = crate::R<DevMisg3Spec>;
#[doc = "Field `MVDMA_CMPLT_CXF` reader - "]
pub type MvdmaCmpltCxfR = crate::BitReader;
#[doc = "Field `MVDMA_CMPLT_F0` reader - "]
pub type MvdmaCmpltF0R = crate::BitReader;
#[doc = "Field `MVDMA_CMPLT_F1` reader - "]
pub type MvdmaCmpltF1R = crate::BitReader;
#[doc = "Field `MVDMA_CMPLT_F2` reader - "]
pub type MvdmaCmpltF2R = crate::BitReader;
#[doc = "Field `MVDMA_CMPLT_F3` reader - "]
pub type MvdmaCmpltF3R = crate::BitReader;
#[doc = "Field `MVDMA_CMPLT_F4` reader - "]
pub type MvdmaCmpltF4R = crate::BitReader;
#[doc = "Field `MVDMA_CMPLT_F5` reader - "]
pub type MvdmaCmpltF5R = crate::BitReader;
#[doc = "Field `MVDMA_CMPLT_F6` reader - "]
pub type MvdmaCmpltF6R = crate::BitReader;
#[doc = "Field `MVDMA_CMPLT_F7` reader - "]
pub type MvdmaCmpltF7R = crate::BitReader;
#[doc = "Field `MVDMA_CMPLT_F8` reader - "]
pub type MvdmaCmpltF8R = crate::BitReader;
#[doc = "Field `MVDMA_CMPLT_F9` reader - "]
pub type MvdmaCmpltF9R = crate::BitReader;
#[doc = "Field `MVDMA_CMPLT_F10` reader - "]
pub type MvdmaCmpltF10R = crate::BitReader;
#[doc = "Field `MVDMA_CMPLT_F11` reader - "]
pub type MvdmaCmpltF11R = crate::BitReader;
#[doc = "Field `MVDMA_CMPLT_F12` reader - "]
pub type MvdmaCmpltF12R = crate::BitReader;
#[doc = "Field `MVDMA_CMPLT_F13` reader - "]
pub type MvdmaCmpltF13R = crate::BitReader;
#[doc = "Field `MVDMA_CMPLT_F14` reader - "]
pub type MvdmaCmpltF14R = crate::BitReader;
#[doc = "Field `MVDMA_ERROR_CXF` reader - "]
pub type MvdmaErrorCxfR = crate::BitReader;
#[doc = "Field `MVDMA_ERROR_F0` reader - "]
pub type MvdmaErrorF0R = crate::BitReader;
#[doc = "Field `MVDMA_ERROR_F1` reader - "]
pub type MvdmaErrorF1R = crate::BitReader;
#[doc = "Field `MVDMA_ERROR_F2` reader - "]
pub type MvdmaErrorF2R = crate::BitReader;
#[doc = "Field `MVDMA_ERROR_F3` reader - "]
pub type MvdmaErrorF3R = crate::BitReader;
#[doc = "Field `MVDMA_ERROR_F4` reader - "]
pub type MvdmaErrorF4R = crate::BitReader;
#[doc = "Field `MVDMA_ERROR_F5` reader - "]
pub type MvdmaErrorF5R = crate::BitReader;
#[doc = "Field `MVDMA_ERROR_F6` reader - "]
pub type MvdmaErrorF6R = crate::BitReader;
#[doc = "Field `MVDMA_ERROR_F7` reader - "]
pub type MvdmaErrorF7R = crate::BitReader;
#[doc = "Field `MVDMA_ERROR_F8` reader - "]
pub type MvdmaErrorF8R = crate::BitReader;
#[doc = "Field `MVDMA_ERROR_F9` reader - "]
pub type MvdmaErrorF9R = crate::BitReader;
#[doc = "Field `MVDMA_ERROR_F10` reader - "]
pub type MvdmaErrorF10R = crate::BitReader;
#[doc = "Field `MVDMA_ERROR_F11` reader - "]
pub type MvdmaErrorF11R = crate::BitReader;
#[doc = "Field `MVDMA_ERROR_F12` reader - "]
pub type MvdmaErrorF12R = crate::BitReader;
#[doc = "Field `MVDMA_ERROR_F13` reader - "]
pub type MvdmaErrorF13R = crate::BitReader;
#[doc = "Field `MVDMA_ERROR_F14` reader - "]
pub type MvdmaErrorF14R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mvdma_cmplt_cxf(&self) -> MvdmaCmpltCxfR {
        MvdmaCmpltCxfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mvdma_cmplt_f0(&self) -> MvdmaCmpltF0R {
        MvdmaCmpltF0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn mvdma_cmplt_f1(&self) -> MvdmaCmpltF1R {
        MvdmaCmpltF1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn mvdma_cmplt_f2(&self) -> MvdmaCmpltF2R {
        MvdmaCmpltF2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mvdma_cmplt_f3(&self) -> MvdmaCmpltF3R {
        MvdmaCmpltF3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn mvdma_cmplt_f4(&self) -> MvdmaCmpltF4R {
        MvdmaCmpltF4R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn mvdma_cmplt_f5(&self) -> MvdmaCmpltF5R {
        MvdmaCmpltF5R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn mvdma_cmplt_f6(&self) -> MvdmaCmpltF6R {
        MvdmaCmpltF6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn mvdma_cmplt_f7(&self) -> MvdmaCmpltF7R {
        MvdmaCmpltF7R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn mvdma_cmplt_f8(&self) -> MvdmaCmpltF8R {
        MvdmaCmpltF8R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn mvdma_cmplt_f9(&self) -> MvdmaCmpltF9R {
        MvdmaCmpltF9R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn mvdma_cmplt_f10(&self) -> MvdmaCmpltF10R {
        MvdmaCmpltF10R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn mvdma_cmplt_f11(&self) -> MvdmaCmpltF11R {
        MvdmaCmpltF11R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn mvdma_cmplt_f12(&self) -> MvdmaCmpltF12R {
        MvdmaCmpltF12R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn mvdma_cmplt_f13(&self) -> MvdmaCmpltF13R {
        MvdmaCmpltF13R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn mvdma_cmplt_f14(&self) -> MvdmaCmpltF14R {
        MvdmaCmpltF14R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mvdma_error_cxf(&self) -> MvdmaErrorCxfR {
        MvdmaErrorCxfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn mvdma_error_f0(&self) -> MvdmaErrorF0R {
        MvdmaErrorF0R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn mvdma_error_f1(&self) -> MvdmaErrorF1R {
        MvdmaErrorF1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn mvdma_error_f2(&self) -> MvdmaErrorF2R {
        MvdmaErrorF2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn mvdma_error_f3(&self) -> MvdmaErrorF3R {
        MvdmaErrorF3R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn mvdma_error_f4(&self) -> MvdmaErrorF4R {
        MvdmaErrorF4R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn mvdma_error_f5(&self) -> MvdmaErrorF5R {
        MvdmaErrorF5R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mvdma_error_f6(&self) -> MvdmaErrorF6R {
        MvdmaErrorF6R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn mvdma_error_f7(&self) -> MvdmaErrorF7R {
        MvdmaErrorF7R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn mvdma_error_f8(&self) -> MvdmaErrorF8R {
        MvdmaErrorF8R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn mvdma_error_f9(&self) -> MvdmaErrorF9R {
        MvdmaErrorF9R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn mvdma_error_f10(&self) -> MvdmaErrorF10R {
        MvdmaErrorF10R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn mvdma_error_f11(&self) -> MvdmaErrorF11R {
        MvdmaErrorF11R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn mvdma_error_f12(&self) -> MvdmaErrorF12R {
        MvdmaErrorF12R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn mvdma_error_f13(&self) -> MvdmaErrorF13R {
        MvdmaErrorF13R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn mvdma_error_f14(&self) -> MvdmaErrorF14R {
        MvdmaErrorF14R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Device Masked Interrupt Status Group 3 (VDMA Completion/Error for F0-F14)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_misg3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevMisg3Spec;
impl crate::RegisterSpec for DevMisg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_misg3::R`](R) reader structure"]
impl crate::Readable for DevMisg3Spec {}
#[doc = "`reset()` method sets DEV_MISG3 to value 0"]
impl crate::Resettable for DevMisg3Spec {}
