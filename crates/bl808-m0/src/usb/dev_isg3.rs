#[doc = "Register `DEV_ISG3` reader"]
pub type R = crate::R<DevIsg3Spec>;
#[doc = "Field `VDMA_CMPLT_CXF` reader - "]
pub type VdmaCmpltCxfR = crate::BitReader;
#[doc = "Field `VDMA_CMPLT_F0` reader - "]
pub type VdmaCmpltF0R = crate::BitReader;
#[doc = "Field `VDMA_CMPLT_F1` reader - "]
pub type VdmaCmpltF1R = crate::BitReader;
#[doc = "Field `VDMA_CMPLT_F2` reader - "]
pub type VdmaCmpltF2R = crate::BitReader;
#[doc = "Field `VDMA_CMPLT_F3` reader - "]
pub type VdmaCmpltF3R = crate::BitReader;
#[doc = "Field `VDMA_CMPLT_F4` reader - "]
pub type VdmaCmpltF4R = crate::BitReader;
#[doc = "Field `VDMA_CMPLT_F5` reader - "]
pub type VdmaCmpltF5R = crate::BitReader;
#[doc = "Field `VDMA_CMPLT_F6` reader - "]
pub type VdmaCmpltF6R = crate::BitReader;
#[doc = "Field `VDMA_CMPLT_F7` reader - "]
pub type VdmaCmpltF7R = crate::BitReader;
#[doc = "Field `VDMA_CMPLT_F8` reader - "]
pub type VdmaCmpltF8R = crate::BitReader;
#[doc = "Field `VDMA_CMPLT_F9` reader - "]
pub type VdmaCmpltF9R = crate::BitReader;
#[doc = "Field `VDMA_CMPLT_F10` reader - "]
pub type VdmaCmpltF10R = crate::BitReader;
#[doc = "Field `VDMA_CMPLT_F11` reader - "]
pub type VdmaCmpltF11R = crate::BitReader;
#[doc = "Field `VDMA_CMPLT_F12` reader - "]
pub type VdmaCmpltF12R = crate::BitReader;
#[doc = "Field `VDMA_CMPLT_F13` reader - "]
pub type VdmaCmpltF13R = crate::BitReader;
#[doc = "Field `VDMA_CMPLT_F14` reader - "]
pub type VdmaCmpltF14R = crate::BitReader;
#[doc = "Field `VDMA_ERROR_CXF` reader - "]
pub type VdmaErrorCxfR = crate::BitReader;
#[doc = "Field `VDMA_ERROR_F0` reader - "]
pub type VdmaErrorF0R = crate::BitReader;
#[doc = "Field `VDMA_ERROR_F1` reader - "]
pub type VdmaErrorF1R = crate::BitReader;
#[doc = "Field `VDMA_ERROR_F2` reader - "]
pub type VdmaErrorF2R = crate::BitReader;
#[doc = "Field `VDMA_ERROR_F3` reader - "]
pub type VdmaErrorF3R = crate::BitReader;
#[doc = "Field `VDMA_ERROR_F4` reader - "]
pub type VdmaErrorF4R = crate::BitReader;
#[doc = "Field `VDMA_ERROR_F5` reader - "]
pub type VdmaErrorF5R = crate::BitReader;
#[doc = "Field `VDMA_ERROR_F6` reader - "]
pub type VdmaErrorF6R = crate::BitReader;
#[doc = "Field `VDMA_ERROR_F7` reader - "]
pub type VdmaErrorF7R = crate::BitReader;
#[doc = "Field `VDMA_ERROR_F8` reader - "]
pub type VdmaErrorF8R = crate::BitReader;
#[doc = "Field `VDMA_ERROR_F9` reader - "]
pub type VdmaErrorF9R = crate::BitReader;
#[doc = "Field `VDMA_ERROR_F10` reader - "]
pub type VdmaErrorF10R = crate::BitReader;
#[doc = "Field `VDMA_ERROR_F11` reader - "]
pub type VdmaErrorF11R = crate::BitReader;
#[doc = "Field `VDMA_ERROR_F12` reader - "]
pub type VdmaErrorF12R = crate::BitReader;
#[doc = "Field `VDMA_ERROR_F13` reader - "]
pub type VdmaErrorF13R = crate::BitReader;
#[doc = "Field `VDMA_ERROR_F14` reader - "]
pub type VdmaErrorF14R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vdma_cmplt_cxf(&self) -> VdmaCmpltCxfR {
        VdmaCmpltCxfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vdma_cmplt_f0(&self) -> VdmaCmpltF0R {
        VdmaCmpltF0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vdma_cmplt_f1(&self) -> VdmaCmpltF1R {
        VdmaCmpltF1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn vdma_cmplt_f2(&self) -> VdmaCmpltF2R {
        VdmaCmpltF2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn vdma_cmplt_f3(&self) -> VdmaCmpltF3R {
        VdmaCmpltF3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn vdma_cmplt_f4(&self) -> VdmaCmpltF4R {
        VdmaCmpltF4R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn vdma_cmplt_f5(&self) -> VdmaCmpltF5R {
        VdmaCmpltF5R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn vdma_cmplt_f6(&self) -> VdmaCmpltF6R {
        VdmaCmpltF6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn vdma_cmplt_f7(&self) -> VdmaCmpltF7R {
        VdmaCmpltF7R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn vdma_cmplt_f8(&self) -> VdmaCmpltF8R {
        VdmaCmpltF8R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn vdma_cmplt_f9(&self) -> VdmaCmpltF9R {
        VdmaCmpltF9R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn vdma_cmplt_f10(&self) -> VdmaCmpltF10R {
        VdmaCmpltF10R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn vdma_cmplt_f11(&self) -> VdmaCmpltF11R {
        VdmaCmpltF11R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn vdma_cmplt_f12(&self) -> VdmaCmpltF12R {
        VdmaCmpltF12R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn vdma_cmplt_f13(&self) -> VdmaCmpltF13R {
        VdmaCmpltF13R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn vdma_cmplt_f14(&self) -> VdmaCmpltF14R {
        VdmaCmpltF14R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn vdma_error_cxf(&self) -> VdmaErrorCxfR {
        VdmaErrorCxfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn vdma_error_f0(&self) -> VdmaErrorF0R {
        VdmaErrorF0R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn vdma_error_f1(&self) -> VdmaErrorF1R {
        VdmaErrorF1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn vdma_error_f2(&self) -> VdmaErrorF2R {
        VdmaErrorF2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn vdma_error_f3(&self) -> VdmaErrorF3R {
        VdmaErrorF3R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn vdma_error_f4(&self) -> VdmaErrorF4R {
        VdmaErrorF4R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn vdma_error_f5(&self) -> VdmaErrorF5R {
        VdmaErrorF5R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn vdma_error_f6(&self) -> VdmaErrorF6R {
        VdmaErrorF6R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn vdma_error_f7(&self) -> VdmaErrorF7R {
        VdmaErrorF7R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn vdma_error_f8(&self) -> VdmaErrorF8R {
        VdmaErrorF8R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn vdma_error_f9(&self) -> VdmaErrorF9R {
        VdmaErrorF9R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn vdma_error_f10(&self) -> VdmaErrorF10R {
        VdmaErrorF10R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn vdma_error_f11(&self) -> VdmaErrorF11R {
        VdmaErrorF11R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn vdma_error_f12(&self) -> VdmaErrorF12R {
        VdmaErrorF12R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn vdma_error_f13(&self) -> VdmaErrorF13R {
        VdmaErrorF13R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn vdma_error_f14(&self) -> VdmaErrorF14R {
        VdmaErrorF14R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Device Interrupt Status Group 3 (VDMA Completion/Error for F0-F14)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_isg3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevIsg3Spec;
impl crate::RegisterSpec for DevIsg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_isg3::R`](R) reader structure"]
impl crate::Readable for DevIsg3Spec {}
#[doc = "`reset()` method sets DEV_ISG3 to value 0"]
impl crate::Resettable for DevIsg3Spec {}
