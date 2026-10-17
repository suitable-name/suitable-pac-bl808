#[doc = "Register `END` reader"]
pub type R = crate::R<EndSpec>;
#[doc = "Register `END` writer"]
pub type W = crate::W<EndSpec>;
#[doc = "Field `addr_end` reader - "]
pub type AddrEndR = crate::FieldReader<u32>;
#[doc = "Field `addr_end` writer - "]
pub type AddrEndW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn addr_end(&self) -> AddrEndR {
        AddrEndR::new(self.bits & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn addr_end(&mut self) -> AddrEndW<'_, EndSpec> {
        AddrEndW::new(self, 0)
    }
}
#[doc = "Region End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EndSpec;
impl crate::RegisterSpec for EndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`end::R`](R) reader structure"]
impl crate::Readable for EndSpec {}
#[doc = "`write(|w| ..)` method takes [`end::W`](W) writer structure"]
impl crate::Writable for EndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets END to value 0"]
impl crate::Resettable for EndSpec {}
