#[doc = "Register `MJPEG_Q_PARAM_%s` reader"]
pub type R = crate::R<MjpegQParam_Spec>;
#[doc = "Register `MJPEG_Q_PARAM_%s` writer"]
pub type W = crate::W<MjpegQParam_Spec>;
#[doc = "Field `reg_q_low` reader - Even-indexed Q parameter (e.g. 0, 2, 4...)"]
pub type RegQLowR = crate::FieldReader<u16>;
#[doc = "Field `reg_q_low` writer - Even-indexed Q parameter (e.g. 0, 2, 4...)"]
pub type RegQLowW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `reg_q_high` reader - Odd-indexed Q parameter (e.g. 1, 3, 5...)"]
pub type RegQHighR = crate::FieldReader<u16>;
#[doc = "Field `reg_q_high` writer - Odd-indexed Q parameter (e.g. 1, 3, 5...)"]
pub type RegQHighW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Even-indexed Q parameter (e.g. 0, 2, 4...)"]
    #[inline(always)]
    pub fn reg_q_low(&self) -> RegQLowR {
        RegQLowR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Odd-indexed Q parameter (e.g. 1, 3, 5...)"]
    #[inline(always)]
    pub fn reg_q_high(&self) -> RegQHighR {
        RegQHighR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Even-indexed Q parameter (e.g. 0, 2, 4...)"]
    #[inline(always)]
    pub fn reg_q_low(&mut self) -> RegQLowW<'_, MjpegQParam_Spec> {
        RegQLowW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Odd-indexed Q parameter (e.g. 1, 3, 5...)"]
    #[inline(always)]
    pub fn reg_q_high(&mut self) -> RegQHighW<'_, MjpegQParam_Spec> {
        RegQHighW::new(self, 16)
    }
}
#[doc = "Quantization Table Parameter Register (index %s)\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_q_param_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_q_param_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegQParam_Spec;
impl crate::RegisterSpec for MjpegQParam_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_q_param_::R`](R) reader structure"]
impl crate::Readable for MjpegQParam_Spec {}
#[doc = "`write(|w| ..)` method takes [`mjpeg_q_param_::W`](W) writer structure"]
impl crate::Writable for MjpegQParam_Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MJPEG_Q_PARAM_%s to value 0"]
impl crate::Resettable for MjpegQParam_Spec {}
