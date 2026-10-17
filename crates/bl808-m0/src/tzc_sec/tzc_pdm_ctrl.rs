#[doc = "Register `TZC_PDM_CTRL` reader"]
pub type R = crate::R<TzcPdmCtrlSpec>;
#[doc = "PDM TrustZone Control (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_pdm_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcPdmCtrlSpec;
impl crate::RegisterSpec for TzcPdmCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_pdm_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcPdmCtrlSpec {}
#[doc = "`reset()` method sets TZC_PDM_CTRL to value 0"]
impl crate::Resettable for TzcPdmCtrlSpec {}
