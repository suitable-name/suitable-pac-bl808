#[doc = "Register `DMA_CLK_CTRL` reader"]
pub type R = crate::R<DmaClkCtrlSpec>;
#[doc = "Register `DMA_CLK_CTRL` writer"]
pub type W = crate::W<DmaClkCtrlSpec>;
#[doc = "Field `dma_clk_en` reader - Enable clocks for DMA channels 0-7."]
pub type DmaClkEnR = crate::FieldReader;
#[doc = "Field `dma_clk_en` writer - Enable clocks for DMA channels 0-7."]
pub type DmaClkEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Enable clocks for DMA channels 0-7."]
    #[inline(always)]
    pub fn dma_clk_en(&self) -> DmaClkEnR {
        DmaClkEnR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Enable clocks for DMA channels 0-7."]
    #[inline(always)]
    pub fn dma_clk_en(&mut self) -> DmaClkEnW<'_, DmaClkCtrlSpec> {
        DmaClkEnW::new(self, 0)
    }
}
#[doc = "Multimedia DMA Clock Gating Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_clk_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_clk_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaClkCtrlSpec;
impl crate::RegisterSpec for DmaClkCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_clk_ctrl::R`](R) reader structure"]
impl crate::Readable for DmaClkCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_clk_ctrl::W`](W) writer structure"]
impl crate::Writable for DmaClkCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CLK_CTRL to value 0xff"]
impl crate::Resettable for DmaClkCtrlSpec {
    const RESET_VALUE: u32 = 0xff;
}
