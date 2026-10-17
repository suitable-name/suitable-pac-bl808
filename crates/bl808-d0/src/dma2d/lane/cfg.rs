#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `CH_EN` reader - Enable channel"]
pub type ChEnR = crate::BitReader;
#[doc = "Field `CH_EN` writer - Enable channel"]
pub type ChEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SrcPeripheral` reader - "]
pub type SrcPeripheralR = crate::FieldReader;
#[doc = "Field `SrcPeripheral` writer - "]
pub type SrcPeripheralW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DstPeripheral` reader - "]
pub type DstPeripheralR = crate::FieldReader;
#[doc = "Field `DstPeripheral` writer - "]
pub type DstPeripheralW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FlowCntrl` reader - Flow control type (M2M, M2P, etc)"]
pub type FlowCntrlR = crate::FieldReader;
#[doc = "Field `FlowCntrl` writer - Flow control type (M2M, M2P, etc)"]
pub type FlowCntrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AHB_B2B` reader - "]
pub type AhbB2bR = crate::BitReader;
#[doc = "Field `AHB_B2B` writer - "]
pub type AhbB2bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_Empty` reader - "]
pub type FifoEmptyR = crate::BitReader;
#[doc = "Field `LLICounter` reader - "]
pub type LlicounterR = crate::FieldReader<u16>;
#[doc = "Field `reg_dma_2d_en` reader - Enable 2D DMA specific features"]
pub type RegDma2dEnR = crate::BitReader;
#[doc = "Field `reg_dma_2d_en` writer - Enable 2D DMA specific features"]
pub type RegDma2dEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable channel"]
    #[inline(always)]
    pub fn ch_en(&self) -> ChEnR {
        ChEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5"]
    #[inline(always)]
    pub fn src_peripheral(&self) -> SrcPeripheralR {
        SrcPeripheralR::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:10"]
    #[inline(always)]
    pub fn dst_peripheral(&self) -> DstPeripheralR {
        DstPeripheralR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:13 - Flow control type (M2M, M2P, etc)"]
    #[inline(always)]
    pub fn flow_cntrl(&self) -> FlowCntrlR {
        FlowCntrlR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ahb_b2b(&self) -> AhbB2bR {
        AhbB2bR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FifoEmptyR {
        FifoEmptyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn llicounter(&self) -> LlicounterR {
        LlicounterR::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Enable 2D DMA specific features"]
    #[inline(always)]
    pub fn reg_dma_2d_en(&self) -> RegDma2dEnR {
        RegDma2dEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable channel"]
    #[inline(always)]
    pub fn ch_en(&mut self) -> ChEnW<'_, CfgSpec> {
        ChEnW::new(self, 0)
    }
    #[doc = "Bits 1:5"]
    #[inline(always)]
    pub fn src_peripheral(&mut self) -> SrcPeripheralW<'_, CfgSpec> {
        SrcPeripheralW::new(self, 1)
    }
    #[doc = "Bits 6:10"]
    #[inline(always)]
    pub fn dst_peripheral(&mut self) -> DstPeripheralW<'_, CfgSpec> {
        DstPeripheralW::new(self, 6)
    }
    #[doc = "Bits 11:13 - Flow control type (M2M, M2P, etc)"]
    #[inline(always)]
    pub fn flow_cntrl(&mut self) -> FlowCntrlW<'_, CfgSpec> {
        FlowCntrlW::new(self, 11)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ahb_b2b(&mut self) -> AhbB2bW<'_, CfgSpec> {
        AhbB2bW::new(self, 14)
    }
    #[doc = "Bit 31 - Enable 2D DMA specific features"]
    #[inline(always)]
    pub fn reg_dma_2d_en(&mut self) -> RegDma2dEnW<'_, CfgSpec> {
        RegDma2dEnW::new(self, 31)
    }
}
#[doc = "Lane Configuration and Flow Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {}
