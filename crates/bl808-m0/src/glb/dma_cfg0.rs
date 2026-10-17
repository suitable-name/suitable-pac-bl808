#[doc = "Register `DMA_CFG0` reader"]
pub type R = crate::R<DmaCfg0Spec>;
#[doc = "Register `DMA_CFG0` writer"]
pub type W = crate::W<DmaCfg0Spec>;
#[doc = "Field `dma_clk_en` reader - "]
pub type DmaClkEnR = crate::FieldReader;
#[doc = "Field `dma_clk_en` writer - "]
pub type DmaClkEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn dma_clk_en(&self) -> DmaClkEnR {
        DmaClkEnR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn dma_clk_en(&mut self) -> DmaClkEnW<'_, DmaCfg0Spec> {
        DmaClkEnW::new(self, 24)
    }
}
#[doc = "DMA0 Channel Clock Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCfg0Spec;
impl crate::RegisterSpec for DmaCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_cfg0::R`](R) reader structure"]
impl crate::Readable for DmaCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_cfg0::W`](W) writer structure"]
impl crate::Writable for DmaCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CFG0 to value 0"]
impl crate::Resettable for DmaCfg0Spec {}
