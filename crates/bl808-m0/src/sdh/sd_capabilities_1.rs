#[doc = "Register `SD_CAPABILITIES_1` reader"]
pub type R = crate::R<SdCapabilities1Spec>;
#[doc = "Field `TIMEOUT_FREQ` reader - "]
pub type TimeoutFreqR = crate::FieldReader;
#[doc = "Field `TIMEOUT_UNIT` reader - "]
pub type TimeoutUnitR = crate::BitReader;
#[doc = "Field `BASE_FREQ` reader - "]
pub type BaseFreqR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn timeout_freq(&self) -> TimeoutFreqR {
        TimeoutFreqR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn timeout_unit(&self) -> TimeoutUnitR {
        TimeoutUnitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn base_freq(&self) -> BaseFreqR {
        BaseFreqR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Host Controller Capabilities Register 1 (Freq/Clock)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_capabilities_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdCapabilities1Spec;
impl crate::RegisterSpec for SdCapabilities1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_capabilities_1::R`](R) reader structure"]
impl crate::Readable for SdCapabilities1Spec {}
#[doc = "`reset()` method sets SD_CAPABILITIES_1 to value 0"]
impl crate::Resettable for SdCapabilities1Spec {}
