#[doc = "Register `DVP_DUMMY_REG` reader"]
pub type R = crate::R<DvpDummyRegSpec>;
#[doc = "Field `DUMMY` reader - Dummy bits (reserved)"]
pub type DummyR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Dummy bits (reserved)"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(self.bits)
    }
}
#[doc = "DVP Dummy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp_dummy_reg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DvpDummyRegSpec;
impl crate::RegisterSpec for DvpDummyRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvp_dummy_reg::R`](R) reader structure"]
impl crate::Readable for DvpDummyRegSpec {}
#[doc = "`reset()` method sets DVP_DUMMY_REG to value 0xf0f0_f0f0"]
impl crate::Resettable for DvpDummyRegSpec {
    const RESET_VALUE: u32 = 0xf0f0_f0f0;
}
