#[doc = "Register `RESET_STS0` reader"]
pub type R = crate::R<ResetSts0Spec>;
#[doc = "Register `RESET_STS0` writer"]
pub type W = crate::W<ResetSts0Spec>;
#[doc = "Field `top_reset_recorder` reader - "]
pub type TopResetRecorderR = crate::FieldReader;
#[doc = "Field `clr_top_reset_recorder` reader - "]
pub type ClrTopResetRecorderR = crate::BitReader;
#[doc = "Field `clr_top_reset_recorder` writer - "]
pub type ClrTopResetRecorderW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn top_reset_recorder(&self) -> TopResetRecorderR {
        TopResetRecorderR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clr_top_reset_recorder(&self) -> ClrTopResetRecorderR {
        ClrTopResetRecorderR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clr_top_reset_recorder(&mut self) -> ClrTopResetRecorderW<'_, ResetSts0Spec> {
        ClrTopResetRecorderW::new(self, 7)
    }
}
#[doc = "Reset Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`reset_sts0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_sts0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetSts0Spec;
impl crate::RegisterSpec for ResetSts0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_sts0::R`](R) reader structure"]
impl crate::Readable for ResetSts0Spec {}
#[doc = "`write(|w| ..)` method takes [`reset_sts0::W`](W) writer structure"]
impl crate::Writable for ResetSts0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESET_STS0 to value 0"]
impl crate::Resettable for ResetSts0Spec {}
