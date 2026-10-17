#[doc = "Register `DMA_Config` reader"]
pub type R = crate::R<DmaConfigSpec>;
#[doc = "Register `DMA_Config` writer"]
pub type W = crate::W<DmaConfigSpec>;
#[doc = "Field `E` reader - SMDMA Enable"]
pub type ER = crate::BitReader;
#[doc = "Field `E` writer - SMDMA Enable"]
pub type EW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ChClkCG` reader - Channel Clock Gating Enable. Bit n corresponds to channel n."]
pub type ChClkCgR = crate::FieldReader;
#[doc = "Field `ChClkCG` writer - Channel Clock Gating Enable. Bit n corresponds to channel n."]
pub type ChClkCgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - SMDMA Enable"]
    #[inline(always)]
    pub fn e(&self) -> ER {
        ER::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:23 - Channel Clock Gating Enable. Bit n corresponds to channel n."]
    #[inline(always)]
    pub fn ch_clk_cg(&self) -> ChClkCgR {
        ChClkCgR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SMDMA Enable"]
    #[inline(always)]
    pub fn e(&mut self) -> EW<'_, DmaConfigSpec> {
        EW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Channel Clock Gating Enable. Bit n corresponds to channel n."]
    #[inline(always)]
    pub fn ch_clk_cg(&mut self) -> ChClkCgW<'_, DmaConfigSpec> {
        ChClkCgW::new(self, 16)
    }
}
#[doc = "DMA Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaConfigSpec;
impl crate::RegisterSpec for DmaConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_config::R`](R) reader structure"]
impl crate::Readable for DmaConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_config::W`](W) writer structure"]
impl crate::Writable for DmaConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_Config to value 0x00ff_0000"]
impl crate::Resettable for DmaConfigSpec {
    const RESET_VALUE: u32 = 0x00ff_0000;
}
