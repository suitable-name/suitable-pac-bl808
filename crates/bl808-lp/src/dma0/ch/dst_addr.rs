#[doc = "Register `DstAddr` reader"]
pub type R = crate::R<DstAddrSpec>;
#[doc = "Register `DstAddr` writer"]
pub type W = crate::W<DstAddrSpec>;
#[doc = "Field `DstAddr` reader - "]
pub type DstAddrR = crate::FieldReader<u32>;
#[doc = "Field `DstAddr` writer - "]
pub type DstAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dst_addr(&self) -> DstAddrR {
        DstAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dst_addr(&mut self) -> DstAddrW<'_, DstAddrSpec> {
        DstAddrW::new(self, 0)
    }
}
#[doc = "DMA destination address\n\nYou can [`read`](crate::Reg::read) this register and get [`dst_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstAddrSpec;
impl crate::RegisterSpec for DstAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_addr::R`](R) reader structure"]
impl crate::Readable for DstAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dst_addr::W`](W) writer structure"]
impl crate::Writable for DstAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DstAddr to value 0"]
impl crate::Resettable for DstAddrSpec {}
