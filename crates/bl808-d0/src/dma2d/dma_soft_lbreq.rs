#[doc = "Register `DMA_SoftLBReq` reader"]
pub type R = crate::R<DmaSoftLbreqSpec>;
#[doc = "Register `DMA_SoftLBReq` writer"]
pub type W = crate::W<DmaSoftLbreqSpec>;
#[doc = "Field `SoftLBReq` reader - "]
pub type SoftLbreqR = crate::FieldReader<u32>;
#[doc = "Field `SoftLBReq` writer - "]
pub type SoftLbreqW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_lbreq(&self) -> SoftLbreqR {
        SoftLbreqR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_lbreq(&mut self) -> SoftLbreqW<'_, DmaSoftLbreqSpec> {
        SoftLbreqW::new(self, 0)
    }
}
#[doc = "DMA Software Last Burst Request\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_soft_lbreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_soft_lbreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaSoftLbreqSpec;
impl crate::RegisterSpec for DmaSoftLbreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_soft_lbreq::R`](R) reader structure"]
impl crate::Readable for DmaSoftLbreqSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_soft_lbreq::W`](W) writer structure"]
impl crate::Writable for DmaSoftLbreqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_SoftLBReq to value 0"]
impl crate::Resettable for DmaSoftLbreqSpec {}
