#[doc = "Register `DEV_MISG0` reader"]
pub type R = crate::R<DevMisg0Spec>;
#[doc = "Field `MCX_SETUP_INT` reader - "]
pub type McxSetupIntR = crate::BitReader;
#[doc = "Field `MCX_IN_INT` reader - "]
pub type McxInIntR = crate::BitReader;
#[doc = "Field `MCX_OUT_INT` reader - "]
pub type McxOutIntR = crate::BitReader;
#[doc = "Field `MCX_COMFAIL_INT` reader - "]
pub type McxComfailIntR = crate::BitReader;
#[doc = "Field `MCX_COMABORT_INT` reader - "]
pub type McxComabortIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mcx_setup_int(&self) -> McxSetupIntR {
        McxSetupIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mcx_in_int(&self) -> McxInIntR {
        McxInIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn mcx_out_int(&self) -> McxOutIntR {
        McxOutIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mcx_comfail_int(&self) -> McxComfailIntR {
        McxComfailIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn mcx_comabort_int(&self) -> McxComabortIntR {
        McxComabortIntR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Device Masked Interrupt Status Group 0 (Control EP)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_misg0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevMisg0Spec;
impl crate::RegisterSpec for DevMisg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_misg0::R`](R) reader structure"]
impl crate::Readable for DevMisg0Spec {}
#[doc = "`reset()` method sets DEV_MISG0 to value 0"]
impl crate::Resettable for DevMisg0Spec {}
