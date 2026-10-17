#[doc = "Register `WVR` reader"]
pub type R = crate::R<WvrSpec>;
#[doc = "Field `wdt_cnt` reader - "]
pub type WdtCntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wdt_cnt(&self) -> WdtCntR {
        WdtCntR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Watchdog Counter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`wvr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WvrSpec;
impl crate::RegisterSpec for WvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wvr::R`](R) reader structure"]
impl crate::Readable for WvrSpec {}
#[doc = "`reset()` method sets WVR to value 0"]
impl crate::Resettable for WvrSpec {}
