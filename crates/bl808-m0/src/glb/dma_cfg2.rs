#[doc = "Register `DMA_CFG2` reader"]
pub type R = crate::R<DmaCfg2Spec>;
#[doc = "Register `DMA_CFG2` writer"]
pub type W = crate::W<DmaCfg2Spec>;
#[doc = "Field `reg_dma_cn_sel` reader - "]
pub type RegDmaCnSelR = crate::FieldReader<u32>;
#[doc = "Field `reg_dma_cn_sel` writer - "]
pub type RegDmaCnSelW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_dma_cn_sel(&self) -> RegDmaCnSelR {
        RegDmaCnSelR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_dma_cn_sel(&mut self) -> RegDmaCnSelW<'_, DmaCfg2Spec> {
        RegDmaCnSelW::new(self, 0)
    }
}
#[doc = "DMA Channel Mux Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCfg2Spec;
impl crate::RegisterSpec for DmaCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_cfg2::R`](R) reader structure"]
impl crate::Readable for DmaCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_cfg2::W`](W) writer structure"]
impl crate::Writable for DmaCfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CFG2 to value 0"]
impl crate::Resettable for DmaCfg2Spec {}
