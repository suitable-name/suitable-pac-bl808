#[doc = "Register `GPIO_CFG143` reader"]
pub type R = crate::R<GpioCfg143Spec>;
#[doc = "Register `GPIO_CFG143` writer"]
pub type W = crate::W<GpioCfg143Spec>;
#[doc = "Field `cr_gpio_dma_tx_en` reader - "]
pub type CrGpioDmaTxEnR = crate::BitReader;
#[doc = "Field `cr_gpio_dma_tx_en` writer - "]
pub type CrGpioDmaTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_gpio_dma_out_sel_latch` reader - "]
pub type CrGpioDmaOutSelLatchR = crate::BitReader;
#[doc = "Field `cr_gpio_dma_out_sel_latch` writer - "]
pub type CrGpioDmaOutSelLatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpio_tx_fifo_clr` writer - "]
pub type GpioTxFifoClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpio_tx_end_clr` writer - "]
pub type GpioTxEndClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpio_tx_fifo_overflow` reader - "]
pub type GpioTxFifoOverflowR = crate::BitReader;
#[doc = "Field `gpio_tx_fifo_underflow` reader - "]
pub type GpioTxFifoUnderflowR = crate::BitReader;
#[doc = "Field `cr_gpio_dma_park_value` reader - "]
pub type CrGpioDmaParkValueR = crate::BitReader;
#[doc = "Field `cr_gpio_dma_park_value` writer - "]
pub type CrGpioDmaParkValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpio_tx_fifo_cnt` reader - "]
pub type GpioTxFifoCntR = crate::FieldReader;
#[doc = "Field `cr_gpio_tx_fifo_th` reader - "]
pub type CrGpioTxFifoThR = crate::FieldReader;
#[doc = "Field `cr_gpio_tx_fifo_th` writer - "]
pub type CrGpioTxFifoThW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `cr_gpio_tx_end_mask` reader - "]
pub type CrGpioTxEndMaskR = crate::BitReader;
#[doc = "Field `cr_gpio_tx_end_mask` writer - "]
pub type CrGpioTxEndMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_gpio_tx_fifo_mask` reader - "]
pub type CrGpioTxFifoMaskR = crate::BitReader;
#[doc = "Field `cr_gpio_tx_fifo_mask` writer - "]
pub type CrGpioTxFifoMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_gpio_tx_fer_mask` reader - "]
pub type CrGpioTxFerMaskR = crate::BitReader;
#[doc = "Field `cr_gpio_tx_fer_mask` writer - "]
pub type CrGpioTxFerMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_gpio_tx_end_int` reader - "]
pub type RGpioTxEndIntR = crate::BitReader;
#[doc = "Field `r_gpio_tx_fifo_int` reader - "]
pub type RGpioTxFifoIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_gpio_dma_tx_en(&self) -> CrGpioDmaTxEnR {
        CrGpioDmaTxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_gpio_dma_out_sel_latch(&self) -> CrGpioDmaOutSelLatchR {
        CrGpioDmaOutSelLatchR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpio_tx_fifo_overflow(&self) -> GpioTxFifoOverflowR {
        GpioTxFifoOverflowR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio_tx_fifo_underflow(&self) -> GpioTxFifoUnderflowR {
        GpioTxFifoUnderflowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_gpio_dma_park_value(&self) -> CrGpioDmaParkValueR {
        CrGpioDmaParkValueR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn gpio_tx_fifo_cnt(&self) -> GpioTxFifoCntR {
        GpioTxFifoCntR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn cr_gpio_tx_fifo_th(&self) -> CrGpioTxFifoThR {
        CrGpioTxFifoThR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cr_gpio_tx_end_mask(&self) -> CrGpioTxEndMaskR {
        CrGpioTxEndMaskR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_gpio_tx_fifo_mask(&self) -> CrGpioTxFifoMaskR {
        CrGpioTxFifoMaskR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_gpio_tx_fer_mask(&self) -> CrGpioTxFerMaskR {
        CrGpioTxFerMaskR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn r_gpio_tx_end_int(&self) -> RGpioTxEndIntR {
        RGpioTxEndIntR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn r_gpio_tx_fifo_int(&self) -> RGpioTxFifoIntR {
        RGpioTxFifoIntR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_gpio_dma_tx_en(&mut self) -> CrGpioDmaTxEnW<'_, GpioCfg143Spec> {
        CrGpioDmaTxEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_gpio_dma_out_sel_latch(&mut self) -> CrGpioDmaOutSelLatchW<'_, GpioCfg143Spec> {
        CrGpioDmaOutSelLatchW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpio_tx_fifo_clr(&mut self) -> GpioTxFifoClrW<'_, GpioCfg143Spec> {
        GpioTxFifoClrW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpio_tx_end_clr(&mut self) -> GpioTxEndClrW<'_, GpioCfg143Spec> {
        GpioTxEndClrW::new(self, 3)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_gpio_dma_park_value(&mut self) -> CrGpioDmaParkValueW<'_, GpioCfg143Spec> {
        CrGpioDmaParkValueW::new(self, 7)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn cr_gpio_tx_fifo_th(&mut self) -> CrGpioTxFifoThW<'_, GpioCfg143Spec> {
        CrGpioTxFifoThW::new(self, 16)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cr_gpio_tx_end_mask(&mut self) -> CrGpioTxEndMaskW<'_, GpioCfg143Spec> {
        CrGpioTxEndMaskW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_gpio_tx_fifo_mask(&mut self) -> CrGpioTxFifoMaskW<'_, GpioCfg143Spec> {
        CrGpioTxFifoMaskW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_gpio_tx_fer_mask(&mut self) -> CrGpioTxFerMaskW<'_, GpioCfg143Spec> {
        CrGpioTxFerMaskW::new(self, 25)
    }
}
#[doc = "GPIO DMA and FIFO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfg143::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfg143::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfg143Spec;
impl crate::RegisterSpec for GpioCfg143Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfg143::R`](R) reader structure"]
impl crate::Readable for GpioCfg143Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfg143::W`](W) writer structure"]
impl crate::Writable for GpioCfg143Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFG143 to value 0"]
impl crate::Resettable for GpioCfg143Spec {}
