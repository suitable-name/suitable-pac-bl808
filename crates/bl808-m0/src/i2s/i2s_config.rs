#[doc = "Register `I2S_CONFIG` reader"]
pub type R = crate::R<I2sConfigSpec>;
#[doc = "Register `I2S_CONFIG` writer"]
pub type W = crate::W<I2sConfigSpec>;
#[doc = "Field `cr_i2s_m_en` reader - I2S Master mode enable"]
pub type CrI2sMEnR = crate::BitReader;
#[doc = "Field `cr_i2s_m_en` writer - I2S Master mode enable"]
pub type CrI2sMEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2s_s_en` reader - I2S Slave mode enable"]
pub type CrI2sSEnR = crate::BitReader;
#[doc = "Field `cr_i2s_s_en` writer - I2S Slave mode enable"]
pub type CrI2sSEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2s_txd_en` reader - I2S TX enable"]
pub type CrI2sTxdEnR = crate::BitReader;
#[doc = "Field `cr_i2s_txd_en` writer - I2S TX enable"]
pub type CrI2sTxdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2s_rxd_en` reader - I2S RX enable"]
pub type CrI2sRxdEnR = crate::BitReader;
#[doc = "Field `cr_i2s_rxd_en` writer - I2S RX enable"]
pub type CrI2sRxdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_mono_mode` reader - Mono mode enable: 0:Stereo, 1:Mono"]
pub type CrMonoModeR = crate::BitReader;
#[doc = "Field `cr_mono_mode` writer - Mono mode enable: 0:Stereo, 1:Mono"]
pub type CrMonoModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_mute_mode` reader - Mute mode enable"]
pub type CrMuteModeR = crate::BitReader;
#[doc = "Field `cr_mute_mode` writer - Mute mode enable"]
pub type CrMuteModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_fs_1t_mode` reader - FS pulse width mode: 0:Slot, 1:1 BCLK"]
pub type CrFs1tModeR = crate::BitReader;
#[doc = "Field `cr_fs_1t_mode` writer - FS pulse width mode: 0:Slot, 1:1 BCLK"]
pub type CrFs1tModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_fs_ch_cnt` reader - FS channel count: 0:2ch, 1:3ch, 2:4ch, 3:6ch"]
pub type CrFsChCntR = crate::FieldReader;
#[doc = "Field `cr_fs_ch_cnt` writer - FS channel count: 0:2ch, 1:3ch, 2:4ch, 3:6ch"]
pub type CrFsChCntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_frame_size` reader - Frame size (cycles per channel): 0:8, 1:16, 2:24, 3:32"]
pub type CrFrameSizeR = crate::FieldReader;
#[doc = "Field `cr_frame_size` writer - Frame size (cycles per channel): 0:8, 1:16, 2:24, 3:32"]
pub type CrFrameSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_data_size` reader - Data size: 0:8, 1:16, 2:24, 3:32"]
pub type CrDataSizeR = crate::FieldReader;
#[doc = "Field `cr_data_size` writer - Data size: 0:8, 1:16, 2:24, 3:32"]
pub type CrDataSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_i2s_mode` reader - I2S mode: 0:Left-Justified, 1:Right-Justified, 2:DSP"]
pub type CrI2sModeR = crate::FieldReader;
#[doc = "Field `cr_i2s_mode` writer - I2S mode: 0:Left-Justified, 1:Right-Justified, 2:DSP"]
pub type CrI2sModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_endian` reader - Endianness: 0:MSB first, 1:LSB first"]
pub type CrEndianR = crate::BitReader;
#[doc = "Field `cr_endian` writer - Endianness: 0:MSB first, 1:LSB first"]
pub type CrEndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_mono_rx_ch` reader - Mono RX channel selection: 0:Left, 1:Right"]
pub type CrMonoRxChR = crate::BitReader;
#[doc = "Field `cr_mono_rx_ch` writer - Mono RX channel selection: 0:Left, 1:Right"]
pub type CrMonoRxChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_ofs_cnt` reader - Data offset cycle count"]
pub type CrOfsCntR = crate::FieldReader;
#[doc = "Field `cr_ofs_cnt` writer - Data offset cycle count"]
pub type CrOfsCntW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `cr_ofs_en` reader - Data offset enable"]
pub type CrOfsEnR = crate::BitReader;
#[doc = "Field `cr_ofs_en` writer - Data offset enable"]
pub type CrOfsEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2S Master mode enable"]
    #[inline(always)]
    pub fn cr_i2s_m_en(&self) -> CrI2sMEnR {
        CrI2sMEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2S Slave mode enable"]
    #[inline(always)]
    pub fn cr_i2s_s_en(&self) -> CrI2sSEnR {
        CrI2sSEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2S TX enable"]
    #[inline(always)]
    pub fn cr_i2s_txd_en(&self) -> CrI2sTxdEnR {
        CrI2sTxdEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2S RX enable"]
    #[inline(always)]
    pub fn cr_i2s_rxd_en(&self) -> CrI2sRxdEnR {
        CrI2sRxdEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mono mode enable: 0:Stereo, 1:Mono"]
    #[inline(always)]
    pub fn cr_mono_mode(&self) -> CrMonoModeR {
        CrMonoModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mute mode enable"]
    #[inline(always)]
    pub fn cr_mute_mode(&self) -> CrMuteModeR {
        CrMuteModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FS pulse width mode: 0:Slot, 1:1 BCLK"]
    #[inline(always)]
    pub fn cr_fs_1t_mode(&self) -> CrFs1tModeR {
        CrFs1tModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - FS channel count: 0:2ch, 1:3ch, 2:4ch, 3:6ch"]
    #[inline(always)]
    pub fn cr_fs_ch_cnt(&self) -> CrFsChCntR {
        CrFsChCntR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Frame size (cycles per channel): 0:8, 1:16, 2:24, 3:32"]
    #[inline(always)]
    pub fn cr_frame_size(&self) -> CrFrameSizeR {
        CrFrameSizeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Data size: 0:8, 1:16, 2:24, 3:32"]
    #[inline(always)]
    pub fn cr_data_size(&self) -> CrDataSizeR {
        CrDataSizeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - I2S mode: 0:Left-Justified, 1:Right-Justified, 2:DSP"]
    #[inline(always)]
    pub fn cr_i2s_mode(&self) -> CrI2sModeR {
        CrI2sModeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Endianness: 0:MSB first, 1:LSB first"]
    #[inline(always)]
    pub fn cr_endian(&self) -> CrEndianR {
        CrEndianR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Mono RX channel selection: 0:Left, 1:Right"]
    #[inline(always)]
    pub fn cr_mono_rx_ch(&self) -> CrMonoRxChR {
        CrMonoRxChR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:24 - Data offset cycle count"]
    #[inline(always)]
    pub fn cr_ofs_cnt(&self) -> CrOfsCntR {
        CrOfsCntR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 25 - Data offset enable"]
    #[inline(always)]
    pub fn cr_ofs_en(&self) -> CrOfsEnR {
        CrOfsEnR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2S Master mode enable"]
    #[inline(always)]
    pub fn cr_i2s_m_en(&mut self) -> CrI2sMEnW<'_, I2sConfigSpec> {
        CrI2sMEnW::new(self, 0)
    }
    #[doc = "Bit 1 - I2S Slave mode enable"]
    #[inline(always)]
    pub fn cr_i2s_s_en(&mut self) -> CrI2sSEnW<'_, I2sConfigSpec> {
        CrI2sSEnW::new(self, 1)
    }
    #[doc = "Bit 2 - I2S TX enable"]
    #[inline(always)]
    pub fn cr_i2s_txd_en(&mut self) -> CrI2sTxdEnW<'_, I2sConfigSpec> {
        CrI2sTxdEnW::new(self, 2)
    }
    #[doc = "Bit 3 - I2S RX enable"]
    #[inline(always)]
    pub fn cr_i2s_rxd_en(&mut self) -> CrI2sRxdEnW<'_, I2sConfigSpec> {
        CrI2sRxdEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Mono mode enable: 0:Stereo, 1:Mono"]
    #[inline(always)]
    pub fn cr_mono_mode(&mut self) -> CrMonoModeW<'_, I2sConfigSpec> {
        CrMonoModeW::new(self, 4)
    }
    #[doc = "Bit 5 - Mute mode enable"]
    #[inline(always)]
    pub fn cr_mute_mode(&mut self) -> CrMuteModeW<'_, I2sConfigSpec> {
        CrMuteModeW::new(self, 5)
    }
    #[doc = "Bit 6 - FS pulse width mode: 0:Slot, 1:1 BCLK"]
    #[inline(always)]
    pub fn cr_fs_1t_mode(&mut self) -> CrFs1tModeW<'_, I2sConfigSpec> {
        CrFs1tModeW::new(self, 6)
    }
    #[doc = "Bits 7:8 - FS channel count: 0:2ch, 1:3ch, 2:4ch, 3:6ch"]
    #[inline(always)]
    pub fn cr_fs_ch_cnt(&mut self) -> CrFsChCntW<'_, I2sConfigSpec> {
        CrFsChCntW::new(self, 7)
    }
    #[doc = "Bits 12:13 - Frame size (cycles per channel): 0:8, 1:16, 2:24, 3:32"]
    #[inline(always)]
    pub fn cr_frame_size(&mut self) -> CrFrameSizeW<'_, I2sConfigSpec> {
        CrFrameSizeW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Data size: 0:8, 1:16, 2:24, 3:32"]
    #[inline(always)]
    pub fn cr_data_size(&mut self) -> CrDataSizeW<'_, I2sConfigSpec> {
        CrDataSizeW::new(self, 14)
    }
    #[doc = "Bits 16:17 - I2S mode: 0:Left-Justified, 1:Right-Justified, 2:DSP"]
    #[inline(always)]
    pub fn cr_i2s_mode(&mut self) -> CrI2sModeW<'_, I2sConfigSpec> {
        CrI2sModeW::new(self, 16)
    }
    #[doc = "Bit 18 - Endianness: 0:MSB first, 1:LSB first"]
    #[inline(always)]
    pub fn cr_endian(&mut self) -> CrEndianW<'_, I2sConfigSpec> {
        CrEndianW::new(self, 18)
    }
    #[doc = "Bit 19 - Mono RX channel selection: 0:Left, 1:Right"]
    #[inline(always)]
    pub fn cr_mono_rx_ch(&mut self) -> CrMonoRxChW<'_, I2sConfigSpec> {
        CrMonoRxChW::new(self, 19)
    }
    #[doc = "Bits 20:24 - Data offset cycle count"]
    #[inline(always)]
    pub fn cr_ofs_cnt(&mut self) -> CrOfsCntW<'_, I2sConfigSpec> {
        CrOfsCntW::new(self, 20)
    }
    #[doc = "Bit 25 - Data offset enable"]
    #[inline(always)]
    pub fn cr_ofs_en(&mut self) -> CrOfsEnW<'_, I2sConfigSpec> {
        CrOfsEnW::new(self, 25)
    }
}
#[doc = "I2S Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sConfigSpec;
impl crate::RegisterSpec for I2sConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_config::R`](R) reader structure"]
impl crate::Readable for I2sConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`i2s_config::W`](W) writer structure"]
impl crate::Writable for I2sConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2S_CONFIG to value 0x5000"]
impl crate::Resettable for I2sConfigSpec {
    const RESET_VALUE: u32 = 0x5000;
}
