#[doc = "Register `DMA_CFG1` reader"]
pub type R = crate::R<DmaCfg1Spec>;
#[doc = "Register `DMA_CFG1` writer"]
pub type W = crate::W<DmaCfg1Spec>;
#[doc = "Field `dma2_clk_en` reader - "]
pub type Dma2ClkEnR = crate::FieldReader;
#[doc = "Field `dma2_clk_en` writer - "]
pub type Dma2ClkEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn dma2_clk_en(&self) -> Dma2ClkEnR {
        Dma2ClkEnR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn dma2_clk_en(&mut self) -> Dma2ClkEnW<'_, DmaCfg1Spec> {
        Dma2ClkEnW::new(self, 24)
    }
}
#[doc = "DMA2 Channel Clock Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCfg1Spec;
impl crate::RegisterSpec for DmaCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_cfg1::R`](R) reader structure"]
impl crate::Readable for DmaCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_cfg1::W`](W) writer structure"]
impl crate::Writable for DmaCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CFG1 to value 0"]
impl crate::Resettable for DmaCfg1Spec {}
