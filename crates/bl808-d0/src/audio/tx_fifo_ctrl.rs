#[doc = "Register `tx_fifo_ctrl` reader"]
pub type R = crate::R<TxFifoCtrlSpec>;
#[doc = "Register `tx_fifo_ctrl` writer"]
pub type W = crate::W<TxFifoCtrlSpec>;
#[doc = "Field `tx_fifo_flush` writer - "]
pub type TxFifoFlushW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `txo_int_en` reader - "]
pub type TxoIntEnR = crate::BitReader;
#[doc = "Field `txo_int_en` writer - "]
pub type TxoIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `txu_int_en` reader - "]
pub type TxuIntEnR = crate::BitReader;
#[doc = "Field `txu_int_en` writer - "]
pub type TxuIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `txa_int_en` reader - "]
pub type TxaIntEnR = crate::BitReader;
#[doc = "Field `txa_int_en` writer - "]
pub type TxaIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_drq_en` reader - "]
pub type TxDrqEnR = crate::BitReader;
#[doc = "Field `tx_drq_en` writer - "]
pub type TxDrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_data_res` reader - "]
pub type TxDataResR = crate::BitReader;
#[doc = "Field `tx_data_res` writer - "]
pub type TxDataResW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_ch_en` reader - "]
pub type TxChEnR = crate::FieldReader;
#[doc = "Field `tx_ch_en` writer - "]
pub type TxChEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tx_drq_cnt` reader - "]
pub type TxDrqCntR = crate::FieldReader;
#[doc = "Field `tx_drq_cnt` writer - "]
pub type TxDrqCntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tx_trg_level` reader - "]
pub type TxTrgLevelR = crate::FieldReader;
#[doc = "Field `tx_trg_level` writer - "]
pub type TxTrgLevelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tx_data_mode` reader - "]
pub type TxDataModeR = crate::FieldReader;
#[doc = "Field `tx_data_mode` writer - "]
pub type TxDataModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txo_int_en(&self) -> TxoIntEnR {
        TxoIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn txu_int_en(&self) -> TxuIntEnR {
        TxuIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn txa_int_en(&self) -> TxaIntEnR {
        TxaIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_drq_en(&self) -> TxDrqEnR {
        TxDrqEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_data_res(&self) -> TxDataResR {
        TxDataResR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tx_ch_en(&self) -> TxChEnR {
        TxChEnR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn tx_drq_cnt(&self) -> TxDrqCntR {
        TxDrqCntR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tx_trg_level(&self) -> TxTrgLevelR {
        TxTrgLevelR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn tx_data_mode(&self) -> TxDataModeR {
        TxDataModeR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_fifo_flush(&mut self) -> TxFifoFlushW<'_, TxFifoCtrlSpec> {
        TxFifoFlushW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txo_int_en(&mut self) -> TxoIntEnW<'_, TxFifoCtrlSpec> {
        TxoIntEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn txu_int_en(&mut self) -> TxuIntEnW<'_, TxFifoCtrlSpec> {
        TxuIntEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn txa_int_en(&mut self) -> TxaIntEnW<'_, TxFifoCtrlSpec> {
        TxaIntEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_drq_en(&mut self) -> TxDrqEnW<'_, TxFifoCtrlSpec> {
        TxDrqEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_data_res(&mut self) -> TxDataResW<'_, TxFifoCtrlSpec> {
        TxDataResW::new(self, 5)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tx_ch_en(&mut self) -> TxChEnW<'_, TxFifoCtrlSpec> {
        TxChEnW::new(self, 8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn tx_drq_cnt(&mut self) -> TxDrqCntW<'_, TxFifoCtrlSpec> {
        TxDrqCntW::new(self, 14)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tx_trg_level(&mut self) -> TxTrgLevelW<'_, TxFifoCtrlSpec> {
        TxTrgLevelW::new(self, 16)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn tx_data_mode(&mut self) -> TxDataModeW<'_, TxFifoCtrlSpec> {
        TxDataModeW::new(self, 24)
    }
}
#[doc = "TX FIFO Control: Configuration for audio data transmission and DMA thresholds\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_fifo_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_fifo_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxFifoCtrlSpec;
impl crate::RegisterSpec for TxFifoCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_fifo_ctrl::R`](R) reader structure"]
impl crate::Readable for TxFifoCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_fifo_ctrl::W`](W) writer structure"]
impl crate::Writable for TxFifoCtrlSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets tx_fifo_ctrl to value 0"]
impl crate::Resettable for TxFifoCtrlSpec {}
