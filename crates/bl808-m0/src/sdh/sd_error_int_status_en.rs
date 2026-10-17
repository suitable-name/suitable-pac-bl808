#[doc = "Register `SD_ERROR_INT_STATUS_EN` reader"]
pub type R = crate::R<SdErrorIntStatusEnSpec>;
#[doc = "Register `SD_ERROR_INT_STATUS_EN` writer"]
pub type W = crate::W<SdErrorIntStatusEnSpec>;
#[doc = "Field `CMD_TIMEOUT_ERR_EN` reader - "]
pub type CmdTimeoutErrEnR = crate::BitReader;
#[doc = "Field `CMD_TIMEOUT_ERR_EN` writer - "]
pub type CmdTimeoutErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_CRC_ERR_EN` reader - "]
pub type CmdCrcErrEnR = crate::BitReader;
#[doc = "Field `CMD_CRC_ERR_EN` writer - "]
pub type CmdCrcErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_END_BIT_ERR_EN` reader - "]
pub type CmdEndBitErrEnR = crate::BitReader;
#[doc = "Field `CMD_END_BIT_ERR_EN` writer - "]
pub type CmdEndBitErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_INDEX_ERR_EN` reader - "]
pub type CmdIndexErrEnR = crate::BitReader;
#[doc = "Field `CMD_INDEX_ERR_EN` writer - "]
pub type CmdIndexErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_TIMEOUT_ERR_EN` reader - "]
pub type DataTimeoutErrEnR = crate::BitReader;
#[doc = "Field `DATA_TIMEOUT_ERR_EN` writer - "]
pub type DataTimeoutErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_DATA_CRC_ERR_EN` reader - "]
pub type RdDataCrcErrEnR = crate::BitReader;
#[doc = "Field `RD_DATA_CRC_ERR_EN` writer - "]
pub type RdDataCrcErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_DATA_END_BIT_ERR_EN` reader - "]
pub type RdDataEndBitErrEnR = crate::BitReader;
#[doc = "Field `RD_DATA_END_BIT_ERR_EN` writer - "]
pub type RdDataEndBitErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUR_LIM_ERR_EN` reader - "]
pub type CurLimErrEnR = crate::BitReader;
#[doc = "Field `CUR_LIM_ERR_EN` writer - "]
pub type CurLimErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_CMD12_ERR_EN` reader - "]
pub type AutoCmd12ErrEnR = crate::BitReader;
#[doc = "Field `AUTO_CMD12_ERR_EN` writer - "]
pub type AutoCmd12ErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMA_ERR_EN` reader - "]
pub type AdmaErrEnR = crate::BitReader;
#[doc = "Field `ADMA_ERR_EN` writer - "]
pub type AdmaErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUNING_ERR_EN` reader - "]
pub type TuningErrEnR = crate::BitReader;
#[doc = "Field `TUNING_ERR_EN` writer - "]
pub type TuningErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_ERR_EN` reader - "]
pub type SpiErrEnR = crate::BitReader;
#[doc = "Field `SPI_ERR_EN` writer - "]
pub type SpiErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_RESP_ERR_EN` reader - "]
pub type AxiRespErrEnR = crate::BitReader;
#[doc = "Field `AXI_RESP_ERR_EN` writer - "]
pub type AxiRespErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPL_TIMEOUT_ERR_EN` reader - "]
pub type CplTimeoutErrEnR = crate::BitReader;
#[doc = "Field `CPL_TIMEOUT_ERR_EN` writer - "]
pub type CplTimeoutErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_STATUS_ERR_EN` reader - "]
pub type CrcStatusErrEnR = crate::BitReader;
#[doc = "Field `CRC_STATUS_ERR_EN` writer - "]
pub type CrcStatusErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmd_timeout_err_en(&self) -> CmdTimeoutErrEnR {
        CmdTimeoutErrEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cmd_crc_err_en(&self) -> CmdCrcErrEnR {
        CmdCrcErrEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cmd_end_bit_err_en(&self) -> CmdEndBitErrEnR {
        CmdEndBitErrEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cmd_index_err_en(&self) -> CmdIndexErrEnR {
        CmdIndexErrEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn data_timeout_err_en(&self) -> DataTimeoutErrEnR {
        DataTimeoutErrEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rd_data_crc_err_en(&self) -> RdDataCrcErrEnR {
        RdDataCrcErrEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rd_data_end_bit_err_en(&self) -> RdDataEndBitErrEnR {
        RdDataEndBitErrEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cur_lim_err_en(&self) -> CurLimErrEnR {
        CurLimErrEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn auto_cmd12_err_en(&self) -> AutoCmd12ErrEnR {
        AutoCmd12ErrEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn adma_err_en(&self) -> AdmaErrEnR {
        AdmaErrEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tuning_err_en(&self) -> TuningErrEnR {
        TuningErrEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi_err_en(&self) -> SpiErrEnR {
        SpiErrEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn axi_resp_err_en(&self) -> AxiRespErrEnR {
        AxiRespErrEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cpl_timeout_err_en(&self) -> CplTimeoutErrEnR {
        CplTimeoutErrEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn crc_status_err_en(&self) -> CrcStatusErrEnR {
        CrcStatusErrEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmd_timeout_err_en(&mut self) -> CmdTimeoutErrEnW<'_, SdErrorIntStatusEnSpec> {
        CmdTimeoutErrEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cmd_crc_err_en(&mut self) -> CmdCrcErrEnW<'_, SdErrorIntStatusEnSpec> {
        CmdCrcErrEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cmd_end_bit_err_en(&mut self) -> CmdEndBitErrEnW<'_, SdErrorIntStatusEnSpec> {
        CmdEndBitErrEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cmd_index_err_en(&mut self) -> CmdIndexErrEnW<'_, SdErrorIntStatusEnSpec> {
        CmdIndexErrEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn data_timeout_err_en(&mut self) -> DataTimeoutErrEnW<'_, SdErrorIntStatusEnSpec> {
        DataTimeoutErrEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rd_data_crc_err_en(&mut self) -> RdDataCrcErrEnW<'_, SdErrorIntStatusEnSpec> {
        RdDataCrcErrEnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rd_data_end_bit_err_en(&mut self) -> RdDataEndBitErrEnW<'_, SdErrorIntStatusEnSpec> {
        RdDataEndBitErrEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cur_lim_err_en(&mut self) -> CurLimErrEnW<'_, SdErrorIntStatusEnSpec> {
        CurLimErrEnW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn auto_cmd12_err_en(&mut self) -> AutoCmd12ErrEnW<'_, SdErrorIntStatusEnSpec> {
        AutoCmd12ErrEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn adma_err_en(&mut self) -> AdmaErrEnW<'_, SdErrorIntStatusEnSpec> {
        AdmaErrEnW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tuning_err_en(&mut self) -> TuningErrEnW<'_, SdErrorIntStatusEnSpec> {
        TuningErrEnW::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi_err_en(&mut self) -> SpiErrEnW<'_, SdErrorIntStatusEnSpec> {
        SpiErrEnW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn axi_resp_err_en(&mut self) -> AxiRespErrEnW<'_, SdErrorIntStatusEnSpec> {
        AxiRespErrEnW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cpl_timeout_err_en(&mut self) -> CplTimeoutErrEnW<'_, SdErrorIntStatusEnSpec> {
        CplTimeoutErrEnW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn crc_status_err_en(&mut self) -> CrcStatusErrEnW<'_, SdErrorIntStatusEnSpec> {
        CrcStatusErrEnW::new(self, 15)
    }
}
#[doc = "Error Interrupt Status Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_error_int_status_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_error_int_status_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdErrorIntStatusEnSpec;
impl crate::RegisterSpec for SdErrorIntStatusEnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_error_int_status_en::R`](R) reader structure"]
impl crate::Readable for SdErrorIntStatusEnSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_error_int_status_en::W`](W) writer structure"]
impl crate::Writable for SdErrorIntStatusEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_ERROR_INT_STATUS_EN to value 0"]
impl crate::Resettable for SdErrorIntStatusEnSpec {}
