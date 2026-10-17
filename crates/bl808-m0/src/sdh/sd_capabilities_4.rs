#[doc = "Register `SD_CAPABILITIES_4` reader"]
pub type R = crate::R<SdCapabilities4Spec>;
#[doc = "Field `CLK_MULTIPLIER` reader - "]
pub type ClkMultiplierR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn clk_multiplier(&self) -> ClkMultiplierR {
        ClkMultiplierR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Host Controller Capabilities Register 4 (Clock Multiplier)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_capabilities_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdCapabilities4Spec;
impl crate::RegisterSpec for SdCapabilities4Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_capabilities_4::R`](R) reader structure"]
impl crate::Readable for SdCapabilities4Spec {}
#[doc = "`reset()` method sets SD_CAPABILITIES_4 to value 0"]
impl crate::Resettable for SdCapabilities4Spec {}
