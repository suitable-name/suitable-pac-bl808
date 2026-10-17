#[doc = "Register `ERROR_WARNING_LIMIT` reader"]
pub type R = crate::R<ErrorWarningLimitSpec>;
#[doc = "Register `ERROR_WARNING_LIMIT` writer"]
pub type W = crate::W<ErrorWarningLimitSpec>;
#[doc = "Field `err_warning_limit` reader - 8-bit error warning threshold"]
pub type ErrWarningLimitR = crate::FieldReader;
#[doc = "Field `err_warning_limit` writer - 8-bit error warning threshold"]
pub type ErrWarningLimitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit error warning threshold"]
    #[inline(always)]
    pub fn err_warning_limit(&self) -> ErrWarningLimitR {
        ErrWarningLimitR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit error warning threshold"]
    #[inline(always)]
    pub fn err_warning_limit(&mut self) -> ErrWarningLimitW<'_, ErrorWarningLimitSpec> {
        ErrWarningLimitW::new(self, 0)
    }
}
#[doc = "Error Warning Limit Register. Defines the threshold for triggering the error warning interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`error_warning_limit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`error_warning_limit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorWarningLimitSpec;
impl crate::RegisterSpec for ErrorWarningLimitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`error_warning_limit::R`](R) reader structure"]
impl crate::Readable for ErrorWarningLimitSpec {}
#[doc = "`write(|w| ..)` method takes [`error_warning_limit::W`](W) writer structure"]
impl crate::Writable for ErrorWarningLimitSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERROR_WARNING_LIMIT to value 0"]
impl crate::Resettable for ErrorWarningLimitSpec {}
