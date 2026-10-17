#[doc = "Register `padc_data` reader"]
pub type R = crate::R<PadcDataSpec>;
#[doc = "Field `padc_raw_data` reader - "]
pub type PadcRawDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_raw_data(&self) -> PadcRawDataR {
        PadcRawDataR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Passthrough ADC Data: 24-bit raw conversion result\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcDataSpec;
impl crate::RegisterSpec for PadcDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_data::R`](R) reader structure"]
impl crate::Readable for PadcDataSpec {}
#[doc = "`reset()` method sets padc_data to value 0"]
impl crate::Resettable for PadcDataSpec {}
