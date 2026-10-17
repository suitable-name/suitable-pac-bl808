#[doc = "Register `CPU_CORE_CFG10` reader"]
pub type R = crate::R<CpuCoreCfg10Spec>;
#[doc = "Field `pico_rtc_cnt_h` reader - "]
pub type PicoRtcCntHR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pico_rtc_cnt_h(&self) -> PicoRtcCntHR {
        PicoRtcCntHR::new(self.bits)
    }
}
#[doc = "PICO RTC Counter High\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_core_cfg10::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuCoreCfg10Spec;
impl crate::RegisterSpec for CpuCoreCfg10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_core_cfg10::R`](R) reader structure"]
impl crate::Readable for CpuCoreCfg10Spec {}
#[doc = "`reset()` method sets CPU_CORE_CFG10 to value 0"]
impl crate::Resettable for CpuCoreCfg10Spec {}
