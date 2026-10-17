#[doc = "Register `GPADC_REG_RAW_RESULT` reader"]
pub type R = crate::R<GpadcRegRawResultSpec>;
#[doc = "Field `gpadc_raw_data` reader - "]
pub type GpadcRawDataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn gpadc_raw_data(&self) -> GpadcRawDataR {
        GpadcRawDataR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "GPADC Raw Conversion Result\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_raw_result::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpadcRegRawResultSpec;
impl crate::RegisterSpec for GpadcRegRawResultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_reg_raw_result::R`](R) reader structure"]
impl crate::Readable for GpadcRegRawResultSpec {}
#[doc = "`reset()` method sets GPADC_REG_RAW_RESULT to value 0"]
impl crate::Resettable for GpadcRegRawResultSpec {}
