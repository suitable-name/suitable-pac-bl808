#[doc = "Register `DMA_CPS1` reader"]
pub type R = crate::R<DmaCps1Spec>;
#[doc = "Register `DMA_CPS1` writer"]
pub type W = crate::W<DmaCps1Spec>;
#[doc = "Field `DMA_START_HOV` reader - DMA Start Trigger (Host Overwrite)"]
pub type DmaStartHovR = crate::BitReader;
#[doc = "Field `DMA_START_HOV` writer - DMA Start Trigger (Host Overwrite)"]
pub type DmaStartHovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_TYPE_HOV` reader - DMA Type (Host Overwrite)"]
pub type DmaTypeHovR = crate::BitReader;
#[doc = "Field `DMA_TYPE_HOV` writer - DMA Type (Host Overwrite)"]
pub type DmaTypeHovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_IO_HOV` reader - DMA Direction (0:Write, 1:Read)"]
pub type DmaIoHovR = crate::BitReader;
#[doc = "Field `DMA_IO_HOV` writer - DMA Direction (0:Write, 1:Read)"]
pub type DmaIoHovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_ABORT_HOV` reader - DMA Abort (Host Overwrite)"]
pub type DmaAbortHovR = crate::BitReader;
#[doc = "Field `DMA_ABORT_HOV` writer - DMA Abort (Host Overwrite)"]
pub type DmaAbortHovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRFIFO_DMAABORT_HOV` reader - Clear FIFO after DMA Abort (Host Overwrite)"]
pub type ClrfifoDmaabortHovR = crate::BitReader;
#[doc = "Field `CLRFIFO_DMAABORT_HOV` writer - Clear FIFO after DMA Abort (Host Overwrite)"]
pub type ClrfifoDmaabortHovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_LEN_HOV` reader - DMA transfer length (17 bits)"]
pub type DmaLenHovR = crate::FieldReader<u32>;
#[doc = "Field `DMA_LEN_HOV` writer - DMA transfer length (17 bits)"]
pub type DmaLenHovW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `R_HPORT_HOV` reader - Read/Write Host Port (4 bits) (Host Overwrite)"]
pub type RHportHovR = crate::FieldReader;
#[doc = "Field `R_HPORT_HOV` writer - Read/Write Host Port (4 bits) (Host Overwrite)"]
pub type RHportHovW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UNDEF_LEN_BURST_HOV` reader - Undefined Length Burst (Host Overwrite)"]
pub type UndefLenBurstHovR = crate::BitReader;
#[doc = "Field `UNDEF_LEN_BURST_HOV` writer - Undefined Length Burst (Host Overwrite)"]
pub type UndefLenBurstHovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_WAKEUP` reader - L1 Suspend Wakeup (Host)"]
pub type L1WakeupR = crate::BitReader;
#[doc = "Field `L1_WAKEUP` writer - L1 Suspend Wakeup (Host)"]
pub type L1WakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVPHY_SUSPEND_HOV` reader - Device PHY Suspend (Host Overwrite)"]
pub type DevphySuspendHovR = crate::BitReader;
#[doc = "Field `DEVPHY_SUSPEND_HOV` writer - Device PHY Suspend (Host Overwrite)"]
pub type DevphySuspendHovW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA Start Trigger (Host Overwrite)"]
    #[inline(always)]
    pub fn dma_start_hov(&self) -> DmaStartHovR {
        DmaStartHovR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Type (Host Overwrite)"]
    #[inline(always)]
    pub fn dma_type_hov(&self) -> DmaTypeHovR {
        DmaTypeHovR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Direction (0:Write, 1:Read)"]
    #[inline(always)]
    pub fn dma_io_hov(&self) -> DmaIoHovR {
        DmaIoHovR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Abort (Host Overwrite)"]
    #[inline(always)]
    pub fn dma_abort_hov(&self) -> DmaAbortHovR {
        DmaAbortHovR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear FIFO after DMA Abort (Host Overwrite)"]
    #[inline(always)]
    pub fn clrfifo_dmaabort_hov(&self) -> ClrfifoDmaabortHovR {
        ClrfifoDmaabortHovR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:24 - DMA transfer length (17 bits)"]
    #[inline(always)]
    pub fn dma_len_hov(&self) -> DmaLenHovR {
        DmaLenHovR::new((self.bits >> 8) & 0x0001_ffff)
    }
    #[doc = "Bits 25:28 - Read/Write Host Port (4 bits) (Host Overwrite)"]
    #[inline(always)]
    pub fn r_hport_hov(&self) -> RHportHovR {
        RHportHovR::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Undefined Length Burst (Host Overwrite)"]
    #[inline(always)]
    pub fn undef_len_burst_hov(&self) -> UndefLenBurstHovR {
        UndefLenBurstHovR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - L1 Suspend Wakeup (Host)"]
    #[inline(always)]
    pub fn l1_wakeup(&self) -> L1WakeupR {
        L1WakeupR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Device PHY Suspend (Host Overwrite)"]
    #[inline(always)]
    pub fn devphy_suspend_hov(&self) -> DevphySuspendHovR {
        DevphySuspendHovR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Start Trigger (Host Overwrite)"]
    #[inline(always)]
    pub fn dma_start_hov(&mut self) -> DmaStartHovW<'_, DmaCps1Spec> {
        DmaStartHovW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Type (Host Overwrite)"]
    #[inline(always)]
    pub fn dma_type_hov(&mut self) -> DmaTypeHovW<'_, DmaCps1Spec> {
        DmaTypeHovW::new(self, 1)
    }
    #[doc = "Bit 2 - DMA Direction (0:Write, 1:Read)"]
    #[inline(always)]
    pub fn dma_io_hov(&mut self) -> DmaIoHovW<'_, DmaCps1Spec> {
        DmaIoHovW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Abort (Host Overwrite)"]
    #[inline(always)]
    pub fn dma_abort_hov(&mut self) -> DmaAbortHovW<'_, DmaCps1Spec> {
        DmaAbortHovW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear FIFO after DMA Abort (Host Overwrite)"]
    #[inline(always)]
    pub fn clrfifo_dmaabort_hov(&mut self) -> ClrfifoDmaabortHovW<'_, DmaCps1Spec> {
        ClrfifoDmaabortHovW::new(self, 4)
    }
    #[doc = "Bits 8:24 - DMA transfer length (17 bits)"]
    #[inline(always)]
    pub fn dma_len_hov(&mut self) -> DmaLenHovW<'_, DmaCps1Spec> {
        DmaLenHovW::new(self, 8)
    }
    #[doc = "Bits 25:28 - Read/Write Host Port (4 bits) (Host Overwrite)"]
    #[inline(always)]
    pub fn r_hport_hov(&mut self) -> RHportHovW<'_, DmaCps1Spec> {
        RHportHovW::new(self, 25)
    }
    #[doc = "Bit 29 - Undefined Length Burst (Host Overwrite)"]
    #[inline(always)]
    pub fn undef_len_burst_hov(&mut self) -> UndefLenBurstHovW<'_, DmaCps1Spec> {
        UndefLenBurstHovW::new(self, 29)
    }
    #[doc = "Bit 30 - L1 Suspend Wakeup (Host)"]
    #[inline(always)]
    pub fn l1_wakeup(&mut self) -> L1WakeupW<'_, DmaCps1Spec> {
        L1WakeupW::new(self, 30)
    }
    #[doc = "Bit 31 - Device PHY Suspend (Host Overwrite)"]
    #[inline(always)]
    pub fn devphy_suspend_hov(&mut self) -> DevphySuspendHovW<'_, DmaCps1Spec> {
        DevphySuspendHovW::new(self, 31)
    }
}
#[doc = "DMA Control Packet Set 1 (Control/Length)\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cps1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cps1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCps1Spec;
impl crate::RegisterSpec for DmaCps1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_cps1::R`](R) reader structure"]
impl crate::Readable for DmaCps1Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_cps1::W`](W) writer structure"]
impl crate::Writable for DmaCps1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CPS1 to value 0"]
impl crate::Resettable for DmaCps1Spec {}
