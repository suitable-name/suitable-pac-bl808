#[doc = "Register `SD_HOST_CTRL_VER` reader"]
pub type R = crate::R<SdHostCtrlVerSpec>;
#[doc = "Field `SD_VER` reader - "]
pub type SdVerR = crate::FieldReader;
#[doc = "Field `VENDOR_VER` reader - "]
pub type VendorVerR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd_ver(&self) -> SdVerR {
        SdVerR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn vendor_ver(&self) -> VendorVerR {
        VendorVerR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Host Controller Version Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_host_ctrl_ver::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdHostCtrlVerSpec;
impl crate::RegisterSpec for SdHostCtrlVerSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_host_ctrl_ver::R`](R) reader structure"]
impl crate::Readable for SdHostCtrlVerSpec {}
#[doc = "`reset()` method sets SD_HOST_CTRL_VER to value 0"]
impl crate::Resettable for SdHostCtrlVerSpec {}
