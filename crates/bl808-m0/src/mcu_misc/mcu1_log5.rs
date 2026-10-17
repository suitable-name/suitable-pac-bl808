#[doc = "Register `MCU1_LOG5` reader"]
pub type R = crate::R<Mcu1Log5Spec>;
#[doc = "Field `sts_mcu1_lockup` reader - E907 CPU lockup status"]
pub type StsMcu1LockupR = crate::BitReader;
#[doc = "Field `sts_mcu1_halted` reader - E907 CPU halted status"]
pub type StsMcu1HaltedR = crate::BitReader;
#[doc = "Field `mcu1_ndm_rstn_req` reader - E907 Non-Debug Mode reset request status"]
pub type Mcu1NdmRstnReqR = crate::BitReader;
#[doc = "Field `mcu1_hart_rstn_req` reader - E907 HART reset request status"]
pub type Mcu1HartRstnReqR = crate::BitReader;
impl R {
    #[doc = "Bit 24 - E907 CPU lockup status"]
    #[inline(always)]
    pub fn sts_mcu1_lockup(&self) -> StsMcu1LockupR {
        StsMcu1LockupR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - E907 CPU halted status"]
    #[inline(always)]
    pub fn sts_mcu1_halted(&self) -> StsMcu1HaltedR {
        StsMcu1HaltedR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - E907 Non-Debug Mode reset request status"]
    #[inline(always)]
    pub fn mcu1_ndm_rstn_req(&self) -> Mcu1NdmRstnReqR {
        Mcu1NdmRstnReqR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - E907 HART reset request status"]
    #[inline(always)]
    pub fn mcu1_hart_rstn_req(&self) -> Mcu1HartRstnReqR {
        Mcu1HartRstnReqR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "E907 Debug Log 5 (Lockup/Halted Status)\n\nYou can [`read`](crate::Reg::read) this register and get [`mcu1_log5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcu1Log5Spec;
impl crate::RegisterSpec for Mcu1Log5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcu1_log5::R`](R) reader structure"]
impl crate::Readable for Mcu1Log5Spec {}
#[doc = "`reset()` method sets MCU1_LOG5 to value 0"]
impl crate::Resettable for Mcu1Log5Spec {}
