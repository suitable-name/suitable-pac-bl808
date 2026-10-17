#[doc = "Register `SF_IF_IAHB_4` reader"]
pub type R = crate::R<SfIfIahb4Spec>;
#[doc = "Register `SF_IF_IAHB_4` writer"]
pub type W = crate::W<SfIfIahb4Spec>;
#[doc = "Field `sf_if_2_cmd_buf_0` reader - "]
pub type SfIf2CmdBuf0R = crate::FieldReader<u32>;
#[doc = "Field `sf_if_2_cmd_buf_0` writer - "]
pub type SfIf2CmdBuf0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_if_2_cmd_buf_0(&self) -> SfIf2CmdBuf0R {
        SfIf2CmdBuf0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_if_2_cmd_buf_0(&mut self) -> SfIf2CmdBuf0W<'_, SfIfIahb4Spec> {
        SfIf2CmdBuf0W::new(self, 0)
    }
}
#[doc = "Instruction AHB Command Buffer 2-0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIfIahb4Spec;
impl crate::RegisterSpec for SfIfIahb4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if_iahb_4::R`](R) reader structure"]
impl crate::Readable for SfIfIahb4Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_if_iahb_4::W`](W) writer structure"]
impl crate::Writable for SfIfIahb4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_IF_IAHB_4 to value 0"]
impl crate::Resettable for SfIfIahb4Spec {}
