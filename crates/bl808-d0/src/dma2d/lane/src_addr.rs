#[doc = "Register `SrcAddr` reader"]
pub type R = crate::R<SrcAddrSpec>;
#[doc = "Register `SrcAddr` writer"]
pub type W = crate::W<SrcAddrSpec>;
#[doc = "Field `SrcAddr` reader - "]
pub type SrcAddrR = crate::FieldReader<u32>;
#[doc = "Field `SrcAddr` writer - "]
pub type SrcAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn src_addr(&self) -> SrcAddrR {
        SrcAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn src_addr(&mut self) -> SrcAddrW<'_, SrcAddrSpec> {
        SrcAddrW::new(self, 0)
    }
}
#[doc = "Source start address\n\nYou can [`read`](crate::Reg::read) this register and get [`src_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcAddrSpec;
impl crate::RegisterSpec for SrcAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_addr::R`](R) reader structure"]
impl crate::Readable for SrcAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`src_addr::W`](W) writer structure"]
impl crate::Writable for SrcAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SrcAddr to value 0"]
impl crate::Resettable for SrcAddrSpec {}
