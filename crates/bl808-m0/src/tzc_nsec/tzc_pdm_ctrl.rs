#[doc = "Register `TZC_PDM_CTRL` reader"]
pub type R = crate::R<TzcPdmCtrlSpec>;
#[doc = "Register `TZC_PDM_CTRL` writer"]
pub type W = crate::W<TzcPdmCtrlSpec>;
impl W {}
#[doc = "PDM Control (Reserved/Placeholder for TZSID)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_pdm_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_pdm_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcPdmCtrlSpec;
impl crate::RegisterSpec for TzcPdmCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_pdm_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcPdmCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_pdm_ctrl::W`](W) writer structure"]
impl crate::Writable for TzcPdmCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_PDM_CTRL to value 0"]
impl crate::Resettable for TzcPdmCtrlSpec {}
