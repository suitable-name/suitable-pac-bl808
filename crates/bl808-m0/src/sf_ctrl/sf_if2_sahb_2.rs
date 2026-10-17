#[doc = "Register `SF_IF2_SAHB_2` reader"]
pub type R = crate::R<SfIf2Sahb2Spec>;
#[doc = "Register `SF_IF2_SAHB_2` writer"]
pub type W = crate::W<SfIf2Sahb2Spec>;
#[doc = "Field `sf_if2_0_cmd_buf_1` reader - "]
pub type SfIf2_0CmdBuf1R = crate::FieldReader<u32>;
#[doc = "Field `sf_if2_0_cmd_buf_1` writer - "]
pub type SfIf2_0CmdBuf1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_if2_0_cmd_buf_1(&self) -> SfIf2_0CmdBuf1R {
        SfIf2_0CmdBuf1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_if2_0_cmd_buf_1(&mut self) -> SfIf2_0CmdBuf1W<'_, SfIf2Sahb2Spec> {
        SfIf2_0CmdBuf1W::new(self, 0)
    }
}
#[doc = "SF2 System AHB Command Buffer 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if2_sahb_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if2_sahb_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIf2Sahb2Spec;
impl crate::RegisterSpec for SfIf2Sahb2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if2_sahb_2::R`](R) reader structure"]
impl crate::Readable for SfIf2Sahb2Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_if2_sahb_2::W`](W) writer structure"]
impl crate::Writable for SfIf2Sahb2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_IF2_SAHB_2 to value 0"]
impl crate::Resettable for SfIf2Sahb2Spec {}
