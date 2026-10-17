#[doc = "Register `SF_IF_IAHB_7` reader"]
pub type R = crate::R<SfIfIahb7Spec>;
#[doc = "Register `SF_IF_IAHB_7` writer"]
pub type W = crate::W<SfIfIahb7Spec>;
#[doc = "Field `sf_if_3_cmd_buf_0` reader - "]
pub type SfIf3CmdBuf0R = crate::FieldReader<u32>;
#[doc = "Field `sf_if_3_cmd_buf_0` writer - "]
pub type SfIf3CmdBuf0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_if_3_cmd_buf_0(&self) -> SfIf3CmdBuf0R {
        SfIf3CmdBuf0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_if_3_cmd_buf_0(&mut self) -> SfIf3CmdBuf0W<'_, SfIfIahb7Spec> {
        SfIf3CmdBuf0W::new(self, 0)
    }
}
#[doc = "Instruction AHB Command Buffer 3-0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIfIahb7Spec;
impl crate::RegisterSpec for SfIfIahb7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if_iahb_7::R`](R) reader structure"]
impl crate::Readable for SfIfIahb7Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_if_iahb_7::W`](W) writer structure"]
impl crate::Writable for SfIfIahb7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_IF_IAHB_7 to value 0"]
impl crate::Resettable for SfIfIahb7Spec {}
