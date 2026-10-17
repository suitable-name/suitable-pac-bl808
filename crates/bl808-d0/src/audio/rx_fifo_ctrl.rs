#[doc = "Register `rx_fifo_ctrl` reader"]
pub type R = crate::R<RxFifoCtrlSpec>;
#[doc = "Register `rx_fifo_ctrl` writer"]
pub type W = crate::W<RxFifoCtrlSpec>;
#[doc = "Field `rx_fifo_flush` writer - "]
pub type RxFifoFlushW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `rxo_int_en` reader - "]
pub type RxoIntEnR = crate::BitReader;
#[doc = "Field `rxo_int_en` writer - "]
pub type RxoIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rxu_int_en` reader - "]
pub type RxuIntEnR = crate::BitReader;
#[doc = "Field `rxu_int_en` writer - "]
pub type RxuIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rxa_int_en` reader - "]
pub type RxaIntEnR = crate::BitReader;
#[doc = "Field `rxa_int_en` writer - "]
pub type RxaIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_drq_en` reader - "]
pub type RxDrqEnR = crate::BitReader;
#[doc = "Field `rx_drq_en` writer - "]
pub type RxDrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_data_res` reader - "]
pub type RxDataResR = crate::BitReader;
#[doc = "Field `rx_data_res` writer - "]
pub type RxDataResW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_ch_en` reader - "]
pub type RxChEnR = crate::FieldReader;
#[doc = "Field `rx_ch_en` writer - "]
pub type RxChEnW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `rx_drq_cnt` reader - "]
pub type RxDrqCntR = crate::FieldReader;
#[doc = "Field `rx_drq_cnt` writer - "]
pub type RxDrqCntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rx_trg_level` reader - "]
pub type RxTrgLevelR = crate::FieldReader;
#[doc = "Field `rx_trg_level` writer - "]
pub type RxTrgLevelW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `rx_data_mode` reader - "]
pub type RxDataModeR = crate::FieldReader;
#[doc = "Field `rx_data_mode` writer - "]
pub type RxDataModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rxo_int_en(&self) -> RxoIntEnR {
        RxoIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxu_int_en(&self) -> RxuIntEnR {
        RxuIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rxa_int_en(&self) -> RxaIntEnR {
        RxaIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rx_drq_en(&self) -> RxDrqEnR {
        RxDrqEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_data_res(&self) -> RxDataResR {
        RxDataResR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn rx_ch_en(&self) -> RxChEnR {
        RxChEnR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn rx_drq_cnt(&self) -> RxDrqCntR {
        RxDrqCntR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn rx_trg_level(&self) -> RxTrgLevelR {
        RxTrgLevelR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rx_data_mode(&self) -> RxDataModeR {
        RxDataModeR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_fifo_flush(&mut self) -> RxFifoFlushW<'_, RxFifoCtrlSpec> {
        RxFifoFlushW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rxo_int_en(&mut self) -> RxoIntEnW<'_, RxFifoCtrlSpec> {
        RxoIntEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxu_int_en(&mut self) -> RxuIntEnW<'_, RxFifoCtrlSpec> {
        RxuIntEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rxa_int_en(&mut self) -> RxaIntEnW<'_, RxFifoCtrlSpec> {
        RxaIntEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rx_drq_en(&mut self) -> RxDrqEnW<'_, RxFifoCtrlSpec> {
        RxDrqEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_data_res(&mut self) -> RxDataResW<'_, RxFifoCtrlSpec> {
        RxDataResW::new(self, 5)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn rx_ch_en(&mut self) -> RxChEnW<'_, RxFifoCtrlSpec> {
        RxChEnW::new(self, 8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn rx_drq_cnt(&mut self) -> RxDrqCntW<'_, RxFifoCtrlSpec> {
        RxDrqCntW::new(self, 14)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn rx_trg_level(&mut self) -> RxTrgLevelW<'_, RxFifoCtrlSpec> {
        RxTrgLevelW::new(self, 16)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rx_data_mode(&mut self) -> RxDataModeW<'_, RxFifoCtrlSpec> {
        RxDataModeW::new(self, 24)
    }
}
#[doc = "RX FIFO Control: Configuration for audio data reception and DMA thresholds\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_fifo_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxFifoCtrlSpec;
impl crate::RegisterSpec for RxFifoCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_fifo_ctrl::R`](R) reader structure"]
impl crate::Readable for RxFifoCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_fifo_ctrl::W`](W) writer structure"]
impl crate::Writable for RxFifoCtrlSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets rx_fifo_ctrl to value 0"]
impl crate::Resettable for RxFifoCtrlSpec {}
