#[doc = "Register `DEV_ISG4` reader"]
pub type R = crate::R<DevIsg4Spec>;
#[doc = "Field `L1_INT` reader - "]
pub type L1IntR = crate::BitReader;
#[doc = "Field `VDMA_CMPLT_F15` reader - "]
pub type VdmaCmpltF15R = crate::BitReader;
#[doc = "Field `VDMA_ERROR_F15` reader - "]
pub type VdmaErrorF15R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn l1_int(&self) -> L1IntR {
        L1IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn vdma_cmplt_f15(&self) -> VdmaCmpltF15R {
        VdmaCmpltF15R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn vdma_error_f15(&self) -> VdmaErrorF15R {
        VdmaErrorF15R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Device Interrupt Status Group 4 (L1/VDMA F15)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_isg4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevIsg4Spec;
impl crate::RegisterSpec for DevIsg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_isg4::R`](R) reader structure"]
impl crate::Readable for DevIsg4Spec {}
#[doc = "`reset()` method sets DEV_ISG4 to value 0"]
impl crate::Resettable for DevIsg4Spec {}
