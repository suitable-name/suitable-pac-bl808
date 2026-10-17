#[doc = "Register `HCSPARAMS` reader"]
pub type R = crate::R<HcsparamsSpec>;
#[doc = "Field `N_PORTS` reader - "]
pub type NPortsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn n_ports(&self) -> NPortsR {
        NPortsR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Host Controller Structural Parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsparams::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcsparamsSpec;
impl crate::RegisterSpec for HcsparamsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcsparams::R`](R) reader structure"]
impl crate::Readable for HcsparamsSpec {}
#[doc = "`reset()` method sets HCSPARAMS to value 0"]
impl crate::Resettable for HcsparamsSpec {}
