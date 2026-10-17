#[doc = "Register `DMA_SoftLSReq` reader"]
pub type R = crate::R<DmaSoftLsreqSpec>;
#[doc = "Register `DMA_SoftLSReq` writer"]
pub type W = crate::W<DmaSoftLsreqSpec>;
#[doc = "Field `SoftLSReq` reader - "]
pub type SoftLsreqR = crate::FieldReader<u32>;
#[doc = "Field `SoftLSReq` writer - "]
pub type SoftLsreqW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_lsreq(&self) -> SoftLsreqR {
        SoftLsreqR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_lsreq(&mut self) -> SoftLsreqW<'_, DmaSoftLsreqSpec> {
        SoftLsreqW::new(self, 0)
    }
}
#[doc = "DMA Software Last Single Request\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_soft_lsreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_soft_lsreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaSoftLsreqSpec;
impl crate::RegisterSpec for DmaSoftLsreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_soft_lsreq::R`](R) reader structure"]
impl crate::Readable for DmaSoftLsreqSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_soft_lsreq::W`](W) writer structure"]
impl crate::Writable for DmaSoftLsreqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_SoftLSReq to value 0"]
impl crate::Resettable for DmaSoftLsreqSpec {}
