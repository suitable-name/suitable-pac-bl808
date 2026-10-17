#[doc = "Register `SF_IF_IAHB_8` reader"]
pub type R = crate::R<SfIfIahb8Spec>;
#[doc = "Register `SF_IF_IAHB_8` writer"]
pub type W = crate::W<SfIfIahb8Spec>;
#[doc = "Field `sf_if_3_cmd_buf_1` reader - "]
pub type SfIf3CmdBuf1R = crate::FieldReader<u32>;
#[doc = "Field `sf_if_3_cmd_buf_1` writer - "]
pub type SfIf3CmdBuf1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_if_3_cmd_buf_1(&self) -> SfIf3CmdBuf1R {
        SfIf3CmdBuf1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_if_3_cmd_buf_1(&mut self) -> SfIf3CmdBuf1W<'_, SfIfIahb8Spec> {
        SfIf3CmdBuf1W::new(self, 0)
    }
}
#[doc = "Instruction AHB Command Buffer 3-1\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIfIahb8Spec;
impl crate::RegisterSpec for SfIfIahb8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if_iahb_8::R`](R) reader structure"]
impl crate::Readable for SfIfIahb8Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_if_iahb_8::W`](W) writer structure"]
impl crate::Writable for SfIfIahb8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_IF_IAHB_8 to value 0"]
impl crate::Resettable for SfIfIahb8Spec {}
