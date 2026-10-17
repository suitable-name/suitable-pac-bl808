#[doc = "Register `MIISTATUS` reader"]
pub type R = crate::R<MiistatusSpec>;
#[doc = "Field `MIIM_LINKFAIL` reader - "]
pub type MiimLinkfailR = crate::BitReader;
#[doc = "Field `MIIM_BUSY` reader - "]
pub type MiimBusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn miim_linkfail(&self) -> MiimLinkfailR {
        MiimLinkfailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn miim_busy(&self) -> MiimBusyR {
        MiimBusyR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "MII Management Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`miistatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiistatusSpec;
impl crate::RegisterSpec for MiistatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`miistatus::R`](R) reader structure"]
impl crate::Readable for MiistatusSpec {}
#[doc = "`reset()` method sets MIISTATUS to value 0"]
impl crate::Resettable for MiistatusSpec {}
