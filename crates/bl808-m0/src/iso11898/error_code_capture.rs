#[doc = "Register `ERROR_CODE_CAPTURE` reader"]
pub type R = crate::R<ErrorCodeCaptureSpec>;
#[doc = "Field `segment4_0` reader - "]
pub type Segment4_0R = crate::FieldReader;
#[doc = "Field `dir` reader - "]
pub type DirR = crate::BitReader;
#[doc = "Field `err_code` reader - "]
pub type ErrCodeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn segment4_0(&self) -> Segment4_0R {
        Segment4_0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn err_code(&self) -> ErrCodeR {
        ErrCodeR::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "Error Code Capture Register\n\nYou can [`read`](crate::Reg::read) this register and get [`error_code_capture::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorCodeCaptureSpec;
impl crate::RegisterSpec for ErrorCodeCaptureSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`error_code_capture::R`](R) reader structure"]
impl crate::Readable for ErrorCodeCaptureSpec {}
#[doc = "`reset()` method sets ERROR_CODE_CAPTURE to value 0"]
impl crate::Resettable for ErrorCodeCaptureSpec {}
