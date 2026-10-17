#[doc = "Register `SD_ADMA_ERROR_STATUS` reader"]
pub type R = crate::R<SdAdmaErrorStatusSpec>;
#[doc = "Register `SD_ADMA_ERROR_STATUS` writer"]
pub type W = crate::W<SdAdmaErrorStatusSpec>;
#[doc = "Field `ADMA_STATE` reader - "]
pub type AdmaStateR = crate::FieldReader;
#[doc = "Field `ADMA_LEN_ERR` reader - "]
pub type AdmaLenErrR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adma_state(&self) -> AdmaStateR {
        AdmaStateR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn adma_len_err(&self) -> AdmaLenErrR {
        AdmaLenErrR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {}
#[doc = "ADMA Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_adma_error_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_adma_error_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdAdmaErrorStatusSpec;
impl crate::RegisterSpec for SdAdmaErrorStatusSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_adma_error_status::R`](R) reader structure"]
impl crate::Readable for SdAdmaErrorStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_adma_error_status::W`](W) writer structure"]
impl crate::Writable for SdAdmaErrorStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_ADMA_ERROR_STATUS to value 0"]
impl crate::Resettable for SdAdmaErrorStatusSpec {}
