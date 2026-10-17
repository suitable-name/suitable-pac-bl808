#[doc = "Register `CPU_CORE_CFG9` reader"]
pub type R = crate::R<CpuCoreCfg9Spec>;
#[doc = "Field `pico_rtc_cnt_l` reader - "]
pub type PicoRtcCntLR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pico_rtc_cnt_l(&self) -> PicoRtcCntLR {
        PicoRtcCntLR::new(self.bits)
    }
}
#[doc = "PICO RTC Counter Low\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_core_cfg9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuCoreCfg9Spec;
impl crate::RegisterSpec for CpuCoreCfg9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_core_cfg9::R`](R) reader structure"]
impl crate::Readable for CpuCoreCfg9Spec {}
#[doc = "`reset()` method sets CPU_CORE_CFG9 to value 0"]
impl crate::Resettable for CpuCoreCfg9Spec {}
