#[doc = "Register `SF_IF_IAHB_1` reader"]
pub type R = crate::R<SfIfIahb1Spec>;
#[doc = "Register `SF_IF_IAHB_1` writer"]
pub type W = crate::W<SfIfIahb1Spec>;
#[doc = "Field `sf_if_1_cmd_buf_0` reader - "]
pub type SfIf1CmdBuf0R = crate::FieldReader<u32>;
#[doc = "Field `sf_if_1_cmd_buf_0` writer - "]
pub type SfIf1CmdBuf0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_if_1_cmd_buf_0(&self) -> SfIf1CmdBuf0R {
        SfIf1CmdBuf0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_if_1_cmd_buf_0(&mut self) -> SfIf1CmdBuf0W<'_, SfIfIahb1Spec> {
        SfIf1CmdBuf0W::new(self, 0)
    }
}
#[doc = "Instruction AHB Command Buffer 1-0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIfIahb1Spec;
impl crate::RegisterSpec for SfIfIahb1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if_iahb_1::R`](R) reader structure"]
impl crate::Readable for SfIfIahb1Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_if_iahb_1::W`](W) writer structure"]
impl crate::Writable for SfIfIahb1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_IF_IAHB_1 to value 0"]
impl crate::Resettable for SfIfIahb1Spec {}
