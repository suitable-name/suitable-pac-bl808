#[doc = "Register `SD_ERROR_INT_STATUS_INT_EN` reader"]
pub type R = crate::R<SdErrorIntStatusIntEnSpec>;
#[doc = "Register `SD_ERROR_INT_STATUS_INT_EN` writer"]
pub type W = crate::W<SdErrorIntStatusIntEnSpec>;
#[doc = "Field `CMD_TIMEOUT_ERR_INT_EN` reader - "]
pub type CmdTimeoutErrIntEnR = crate::BitReader;
#[doc = "Field `CMD_TIMEOUT_ERR_INT_EN` writer - "]
pub type CmdTimeoutErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_CRC_ERR_INT_EN` reader - "]
pub type CmdCrcErrIntEnR = crate::BitReader;
#[doc = "Field `CMD_CRC_ERR_INT_EN` writer - "]
pub type CmdCrcErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_END_BIT_ERR_INT_EN` reader - "]
pub type CmdEndBitErrIntEnR = crate::BitReader;
#[doc = "Field `CMD_END_BIT_ERR_INT_EN` writer - "]
pub type CmdEndBitErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_INDEX_ERR_INT_EN` reader - "]
pub type CmdIndexErrIntEnR = crate::BitReader;
#[doc = "Field `CMD_INDEX_ERR_INT_EN` writer - "]
pub type CmdIndexErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_TIMEOUT_ERR_INT_EN` reader - "]
pub type DataTimeoutErrIntEnR = crate::BitReader;
#[doc = "Field `DATA_TIMEOUT_ERR_INT_EN` writer - "]
pub type DataTimeoutErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_DATA_CRC_ERR_INT_EN` reader - "]
pub type RdDataCrcErrIntEnR = crate::BitReader;
#[doc = "Field `RD_DATA_CRC_ERR_INT_EN` writer - "]
pub type RdDataCrcErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_DATA_END_BIT_ERR_INT_EN` reader - "]
pub type RdDataEndBitErrIntEnR = crate::BitReader;
#[doc = "Field `RD_DATA_END_BIT_ERR_INT_EN` writer - "]
pub type RdDataEndBitErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUR_LIM_ERR_INT_EN` reader - "]
pub type CurLimErrIntEnR = crate::BitReader;
#[doc = "Field `CUR_LIM_ERR_INT_EN` writer - "]
pub type CurLimErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_CMD12_ERR_INT_EN` reader - "]
pub type AutoCmd12ErrIntEnR = crate::BitReader;
#[doc = "Field `AUTO_CMD12_ERR_INT_EN` writer - "]
pub type AutoCmd12ErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMA_ERR_INT_EN` reader - "]
pub type AdmaErrIntEnR = crate::BitReader;
#[doc = "Field `ADMA_ERR_INT_EN` writer - "]
pub type AdmaErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUNE_ERR_INT_EN` reader - "]
pub type TuneErrIntEnR = crate::BitReader;
#[doc = "Field `TUNE_ERR_INT_EN` writer - "]
pub type TuneErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_ERR_INT_EN` reader - "]
pub type SpiErrIntEnR = crate::BitReader;
#[doc = "Field `SPI_ERR_INT_EN` writer - "]
pub type SpiErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_RESP_ERR_INT_EN` reader - "]
pub type AxiRespErrIntEnR = crate::BitReader;
#[doc = "Field `AXI_RESP_ERR_INT_EN` writer - "]
pub type AxiRespErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPL_TIMEOUT_ERR_INT_EN` reader - "]
pub type CplTimeoutErrIntEnR = crate::BitReader;
#[doc = "Field `CPL_TIMEOUT_ERR_INT_EN` writer - "]
pub type CplTimeoutErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_STATUS_ERR_INT_EN` reader - "]
pub type CrcStatusErrIntEnR = crate::BitReader;
#[doc = "Field `CRC_STATUS_ERR_INT_EN` writer - "]
pub type CrcStatusErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmd_timeout_err_int_en(&self) -> CmdTimeoutErrIntEnR {
        CmdTimeoutErrIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cmd_crc_err_int_en(&self) -> CmdCrcErrIntEnR {
        CmdCrcErrIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cmd_end_bit_err_int_en(&self) -> CmdEndBitErrIntEnR {
        CmdEndBitErrIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cmd_index_err_int_en(&self) -> CmdIndexErrIntEnR {
        CmdIndexErrIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn data_timeout_err_int_en(&self) -> DataTimeoutErrIntEnR {
        DataTimeoutErrIntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rd_data_crc_err_int_en(&self) -> RdDataCrcErrIntEnR {
        RdDataCrcErrIntEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rd_data_end_bit_err_int_en(&self) -> RdDataEndBitErrIntEnR {
        RdDataEndBitErrIntEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cur_lim_err_int_en(&self) -> CurLimErrIntEnR {
        CurLimErrIntEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn auto_cmd12_err_int_en(&self) -> AutoCmd12ErrIntEnR {
        AutoCmd12ErrIntEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn adma_err_int_en(&self) -> AdmaErrIntEnR {
        AdmaErrIntEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tune_err_int_en(&self) -> TuneErrIntEnR {
        TuneErrIntEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi_err_int_en(&self) -> SpiErrIntEnR {
        SpiErrIntEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn axi_resp_err_int_en(&self) -> AxiRespErrIntEnR {
        AxiRespErrIntEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cpl_timeout_err_int_en(&self) -> CplTimeoutErrIntEnR {
        CplTimeoutErrIntEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn crc_status_err_int_en(&self) -> CrcStatusErrIntEnR {
        CrcStatusErrIntEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmd_timeout_err_int_en(&mut self) -> CmdTimeoutErrIntEnW<'_, SdErrorIntStatusIntEnSpec> {
        CmdTimeoutErrIntEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cmd_crc_err_int_en(&mut self) -> CmdCrcErrIntEnW<'_, SdErrorIntStatusIntEnSpec> {
        CmdCrcErrIntEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cmd_end_bit_err_int_en(&mut self) -> CmdEndBitErrIntEnW<'_, SdErrorIntStatusIntEnSpec> {
        CmdEndBitErrIntEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cmd_index_err_int_en(&mut self) -> CmdIndexErrIntEnW<'_, SdErrorIntStatusIntEnSpec> {
        CmdIndexErrIntEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn data_timeout_err_int_en(
        &mut self,
    ) -> DataTimeoutErrIntEnW<'_, SdErrorIntStatusIntEnSpec> {
        DataTimeoutErrIntEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rd_data_crc_err_int_en(&mut self) -> RdDataCrcErrIntEnW<'_, SdErrorIntStatusIntEnSpec> {
        RdDataCrcErrIntEnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rd_data_end_bit_err_int_en(
        &mut self,
    ) -> RdDataEndBitErrIntEnW<'_, SdErrorIntStatusIntEnSpec> {
        RdDataEndBitErrIntEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cur_lim_err_int_en(&mut self) -> CurLimErrIntEnW<'_, SdErrorIntStatusIntEnSpec> {
        CurLimErrIntEnW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn auto_cmd12_err_int_en(&mut self) -> AutoCmd12ErrIntEnW<'_, SdErrorIntStatusIntEnSpec> {
        AutoCmd12ErrIntEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn adma_err_int_en(&mut self) -> AdmaErrIntEnW<'_, SdErrorIntStatusIntEnSpec> {
        AdmaErrIntEnW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tune_err_int_en(&mut self) -> TuneErrIntEnW<'_, SdErrorIntStatusIntEnSpec> {
        TuneErrIntEnW::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi_err_int_en(&mut self) -> SpiErrIntEnW<'_, SdErrorIntStatusIntEnSpec> {
        SpiErrIntEnW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn axi_resp_err_int_en(&mut self) -> AxiRespErrIntEnW<'_, SdErrorIntStatusIntEnSpec> {
        AxiRespErrIntEnW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cpl_timeout_err_int_en(&mut self) -> CplTimeoutErrIntEnW<'_, SdErrorIntStatusIntEnSpec> {
        CplTimeoutErrIntEnW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn crc_status_err_int_en(&mut self) -> CrcStatusErrIntEnW<'_, SdErrorIntStatusIntEnSpec> {
        CrcStatusErrIntEnW::new(self, 15)
    }
}
#[doc = "Error Interrupt Signal Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_error_int_status_int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_error_int_status_int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdErrorIntStatusIntEnSpec;
impl crate::RegisterSpec for SdErrorIntStatusIntEnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_error_int_status_int_en::R`](R) reader structure"]
impl crate::Readable for SdErrorIntStatusIntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_error_int_status_int_en::W`](W) writer structure"]
impl crate::Writable for SdErrorIntStatusIntEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_ERROR_INT_STATUS_INT_EN to value 0"]
impl crate::Resettable for SdErrorIntStatusIntEnSpec {}
