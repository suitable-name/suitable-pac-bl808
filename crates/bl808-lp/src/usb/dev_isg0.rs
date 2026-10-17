#[doc = "Register `DEV_ISG0` reader"]
pub type R = crate::R<DevIsg0Spec>;
#[doc = "Field `CX_SETUP_INT` reader - "]
pub type CxSetupIntR = crate::BitReader;
#[doc = "Field `CX_IN_INT` reader - "]
pub type CxInIntR = crate::BitReader;
#[doc = "Field `CX_OUT_INT` reader - "]
pub type CxOutIntR = crate::BitReader;
#[doc = "Field `CX_COMFAIL_INT` reader - "]
pub type CxComfailIntR = crate::BitReader;
#[doc = "Field `CX_COMABT_INT` reader - "]
pub type CxComabtIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cx_setup_int(&self) -> CxSetupIntR {
        CxSetupIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cx_in_int(&self) -> CxInIntR {
        CxInIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cx_out_int(&self) -> CxOutIntR {
        CxOutIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cx_comfail_int(&self) -> CxComfailIntR {
        CxComfailIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cx_comabt_int(&self) -> CxComabtIntR {
        CxComabtIntR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Device Interrupt Status Group 0 (Control EP)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_isg0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevIsg0Spec;
impl crate::RegisterSpec for DevIsg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_isg0::R`](R) reader structure"]
impl crate::Readable for DevIsg0Spec {}
#[doc = "`reset()` method sets DEV_ISG0 to value 0"]
impl crate::Resettable for DevIsg0Spec {}
