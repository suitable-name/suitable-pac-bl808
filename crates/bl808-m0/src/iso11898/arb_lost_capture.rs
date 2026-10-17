#[doc = "Register `ARB_LOST_CAPTURE` reader"]
pub type R = crate::R<ArbLostCaptureSpec>;
#[doc = "Field `ALC4_0` reader - "]
pub type Alc4_0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn alc4_0(&self) -> Alc4_0R {
        Alc4_0R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Arbitration Lost Capture Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_lost_capture::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArbLostCaptureSpec;
impl crate::RegisterSpec for ArbLostCaptureSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_lost_capture::R`](R) reader structure"]
impl crate::Readable for ArbLostCaptureSpec {}
#[doc = "`reset()` method sets ARB_LOST_CAPTURE to value 0"]
impl crate::Resettable for ArbLostCaptureSpec {}
