#[doc = "Register `DEV_MISG4` reader"]
pub type R = crate::R<DevMisg4Spec>;
#[doc = "Field `ML1_INT` reader - "]
pub type Ml1IntR = crate::BitReader;
#[doc = "Field `MVDMA_CMPLT_F15` reader - "]
pub type MvdmaCmpltF15R = crate::BitReader;
#[doc = "Field `MVDMA_ERROR_F15` reader - "]
pub type MvdmaErrorF15R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ml1_int(&self) -> Ml1IntR {
        Ml1IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mvdma_cmplt_f15(&self) -> MvdmaCmpltF15R {
        MvdmaCmpltF15R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn mvdma_error_f15(&self) -> MvdmaErrorF15R {
        MvdmaErrorF15R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Device Masked Interrupt Status Group 4 (L1/VDMA F15)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_misg4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevMisg4Spec;
impl crate::RegisterSpec for DevMisg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_misg4::R`](R) reader structure"]
impl crate::Readable for DevMisg4Spec {}
#[doc = "`reset()` method sets DEV_MISG4 to value 0"]
impl crate::Resettable for DevMisg4Spec {}
