#[doc = "Register `TZC_SE_CTRL_1` reader"]
pub type R = crate::R<TzcSeCtrl1Spec>;
#[doc = "Field `tzc_sf_cr_tzsid_en` reader - SF Control Register TZID Enable"]
pub type TzcSfCrTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_sf_sec_tzsid_en` reader - SF Security Register TZID Enable"]
pub type TzcSfSecTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsid_crmd` reader - SF Command Mode"]
pub type TzcSfTzsidCrmdR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - SF Control Register TZID Enable"]
    #[inline(always)]
    pub fn tzc_sf_cr_tzsid_en(&self) -> TzcSfCrTzsidEnR {
        TzcSfCrTzsidEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SF Security Register TZID Enable"]
    #[inline(always)]
    pub fn tzc_sf_sec_tzsid_en(&self) -> TzcSfSecTzsidEnR {
        TzcSfSecTzsidEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SF Command Mode"]
    #[inline(always)]
    pub fn tzc_sf_tzsid_crmd(&self) -> TzcSfTzsidCrmdR {
        TzcSfTzsidCrmdR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Security Engine Control 1 (Flash/SF Controls)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_se_ctrl_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcSeCtrl1Spec;
impl crate::RegisterSpec for TzcSeCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_se_ctrl_1::R`](R) reader structure"]
impl crate::Readable for TzcSeCtrl1Spec {}
#[doc = "`reset()` method sets TZC_SE_CTRL_1 to value 0"]
impl crate::Resettable for TzcSeCtrl1Spec {}
