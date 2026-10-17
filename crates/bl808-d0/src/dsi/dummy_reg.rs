#[doc = "Register `DUMMY_REG` reader"]
pub type R = crate::R<DummyRegSpec>;
#[doc = "Register `DUMMY_REG` writer"]
pub type W = crate::W<DummyRegSpec>;
#[doc = "Field `dummy_reg` reader - "]
pub type DummyRegR = crate::FieldReader<u32>;
#[doc = "Field `dummy_reg` writer - "]
pub type DummyRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dummy_reg(&self) -> DummyRegR {
        DummyRegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dummy_reg(&mut self) -> DummyRegW<'_, DummyRegSpec> {
        DummyRegW::new(self, 0)
    }
}
#[doc = "DSI Dummy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dummy_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dummy_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DummyRegSpec;
impl crate::RegisterSpec for DummyRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dummy_reg::R`](R) reader structure"]
impl crate::Readable for DummyRegSpec {}
#[doc = "`write(|w| ..)` method takes [`dummy_reg::W`](W) writer structure"]
impl crate::Writable for DummyRegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DUMMY_REG to value 0"]
impl crate::Resettable for DummyRegSpec {}
