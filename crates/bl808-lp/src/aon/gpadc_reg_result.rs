#[doc = "Register `GPADC_REG_RESULT` reader"]
pub type R = crate::R<GpadcRegResultSpec>;
#[doc = "Field `gpadc_data_out` reader - "]
pub type GpadcDataOutR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpadc_data_out(&self) -> GpadcDataOutR {
        GpadcDataOutR::new(self.bits & 0x03ff_ffff)
    }
}
#[doc = "GPADC Conversion Result\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_result::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpadcRegResultSpec;
impl crate::RegisterSpec for GpadcRegResultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_reg_result::R`](R) reader structure"]
impl crate::Readable for GpadcRegResultSpec {}
#[doc = "`reset()` method sets GPADC_REG_RESULT to value 0"]
impl crate::Resettable for GpadcRegResultSpec {}
