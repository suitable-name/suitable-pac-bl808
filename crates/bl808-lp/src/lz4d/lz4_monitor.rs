#[doc = "Register `LZ4_MONITOR` reader"]
pub type R = crate::R<Lz4MonitorSpec>;
#[doc = "Field `lz4_cs` reader - LZ4 Control State monitor"]
pub type Lz4CsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - LZ4 Control State monitor"]
    #[inline(always)]
    pub fn lz4_cs(&self) -> Lz4CsR {
        Lz4CsR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "LZ4 Engine Monitor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lz4_monitor::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lz4MonitorSpec;
impl crate::RegisterSpec for Lz4MonitorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lz4_monitor::R`](R) reader structure"]
impl crate::Readable for Lz4MonitorSpec {}
#[doc = "`reset()` method sets LZ4_MONITOR to value 0"]
impl crate::Resettable for Lz4MonitorSpec {}
