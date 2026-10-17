#[doc = "Register `GNR_BUF_STATUS` reader"]
pub type R = crate::R<GnrBufStatusSpec>;
#[doc = "Field `st_gnr_fifo_cnt` reader - Generic FIFO data count"]
pub type StGnrFifoCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Generic FIFO data count"]
    #[inline(always)]
    pub fn st_gnr_fifo_cnt(&self) -> StGnrFifoCntR {
        StGnrFifoCntR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Generic Buffer Status\n\nYou can [`read`](crate::Reg::read) this register and get [`gnr_buf_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GnrBufStatusSpec;
impl crate::RegisterSpec for GnrBufStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gnr_buf_status::R`](R) reader structure"]
impl crate::Readable for GnrBufStatusSpec {}
#[doc = "`reset()` method sets GNR_BUF_STATUS to value 0"]
impl crate::Resettable for GnrBufStatusSpec {}
