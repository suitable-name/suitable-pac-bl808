#[doc = "Register `HCCPARAMS` reader"]
pub type R = crate::R<HccparamsSpec>;
#[doc = "Field `PROG_FR_LIST_FLAG` reader - "]
pub type ProgFrListFlagR = crate::BitReader;
#[doc = "Field `ASYN_SCH_PARK_CAP` reader - "]
pub type AsynSchParkCapR = crate::BitReader;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn prog_fr_list_flag(&self) -> ProgFrListFlagR {
        ProgFrListFlagR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn asyn_sch_park_cap(&self) -> AsynSchParkCapR {
        AsynSchParkCapR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Host Controller Capability Parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`hccparams::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HccparamsSpec;
impl crate::RegisterSpec for HccparamsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hccparams::R`](R) reader structure"]
impl crate::Readable for HccparamsSpec {}
#[doc = "`reset()` method sets HCCPARAMS to value 0"]
impl crate::Resettable for HccparamsSpec {}
