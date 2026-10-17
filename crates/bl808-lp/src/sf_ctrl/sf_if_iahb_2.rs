#[doc = "Register `SF_IF_IAHB_2` reader"]
pub type R = crate::R<SfIfIahb2Spec>;
#[doc = "Register `SF_IF_IAHB_2` writer"]
pub type W = crate::W<SfIfIahb2Spec>;
#[doc = "Field `sf_if_1_cmd_buf_1` reader - "]
pub type SfIf1CmdBuf1R = crate::FieldReader<u32>;
#[doc = "Field `sf_if_1_cmd_buf_1` writer - "]
pub type SfIf1CmdBuf1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_if_1_cmd_buf_1(&self) -> SfIf1CmdBuf1R {
        SfIf1CmdBuf1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_if_1_cmd_buf_1(&mut self) -> SfIf1CmdBuf1W<'_, SfIfIahb2Spec> {
        SfIf1CmdBuf1W::new(self, 0)
    }
}
#[doc = "Instruction AHB Command Buffer 1-1\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIfIahb2Spec;
impl crate::RegisterSpec for SfIfIahb2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if_iahb_2::R`](R) reader structure"]
impl crate::Readable for SfIfIahb2Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_if_iahb_2::W`](W) writer structure"]
impl crate::Writable for SfIfIahb2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_IF_IAHB_2 to value 0"]
impl crate::Resettable for SfIfIahb2Spec {}
