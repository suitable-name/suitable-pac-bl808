#[doc = "Register `SF_IF2_SAHB_1` reader"]
pub type R = crate::R<SfIf2Sahb1Spec>;
#[doc = "Register `SF_IF2_SAHB_1` writer"]
pub type W = crate::W<SfIf2Sahb1Spec>;
#[doc = "Field `sf_if2_0_cmd_buf_0` reader - "]
pub type SfIf2_0CmdBuf0R = crate::FieldReader<u32>;
#[doc = "Field `sf_if2_0_cmd_buf_0` writer - "]
pub type SfIf2_0CmdBuf0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_if2_0_cmd_buf_0(&self) -> SfIf2_0CmdBuf0R {
        SfIf2_0CmdBuf0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_if2_0_cmd_buf_0(&mut self) -> SfIf2_0CmdBuf0W<'_, SfIf2Sahb1Spec> {
        SfIf2_0CmdBuf0W::new(self, 0)
    }
}
#[doc = "SF2 System AHB Command Buffer 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if2_sahb_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if2_sahb_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIf2Sahb1Spec;
impl crate::RegisterSpec for SfIf2Sahb1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if2_sahb_1::R`](R) reader structure"]
impl crate::Readable for SfIf2Sahb1Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_if2_sahb_1::W`](W) writer structure"]
impl crate::Writable for SfIf2Sahb1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_IF2_SAHB_1 to value 0"]
impl crate::Resettable for SfIf2Sahb1Spec {}
