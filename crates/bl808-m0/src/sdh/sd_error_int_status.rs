#[doc = "Register `SD_ERROR_INT_STATUS` reader"]
pub type R = crate::R<SdErrorIntStatusSpec>;
#[doc = "Register `SD_ERROR_INT_STATUS` writer"]
pub type W = crate::W<SdErrorIntStatusSpec>;
#[doc = "Field `CMD_TIMEOUT_ERR` reader - "]
pub type CmdTimeoutErrR = crate::BitReader;
#[doc = "Field `CMD_TIMEOUT_ERR` writer - "]
pub type CmdTimeoutErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CMD_CRC_ERR` reader - "]
pub type CmdCrcErrR = crate::BitReader;
#[doc = "Field `CMD_CRC_ERR` writer - "]
pub type CmdCrcErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CMD_END_BIT_ERR` reader - "]
pub type CmdEndBitErrR = crate::BitReader;
#[doc = "Field `CMD_END_BIT_ERR` writer - "]
pub type CmdEndBitErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CMD_INDEX_ERR` reader - "]
pub type CmdIndexErrR = crate::BitReader;
#[doc = "Field `CMD_INDEX_ERR` writer - "]
pub type CmdIndexErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DATA_TIMEOUT_ERR` reader - "]
pub type DataTimeoutErrR = crate::BitReader;
#[doc = "Field `DATA_TIMEOUT_ERR` writer - "]
pub type DataTimeoutErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RD_DATA_CRC_ERR` reader - "]
pub type RdDataCrcErrR = crate::BitReader;
#[doc = "Field `RD_DATA_CRC_ERR` writer - "]
pub type RdDataCrcErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RD_DATA_END_BIT_ERR` reader - "]
pub type RdDataEndBitErrR = crate::BitReader;
#[doc = "Field `RD_DATA_END_BIT_ERR` writer - "]
pub type RdDataEndBitErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CUR_LIMIT_ERR` reader - "]
pub type CurLimitErrR = crate::BitReader;
#[doc = "Field `CUR_LIMIT_ERR` writer - "]
pub type CurLimitErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AUTO_CMD12_ERR` reader - "]
pub type AutoCmd12ErrR = crate::BitReader;
#[doc = "Field `AUTO_CMD12_ERR` writer - "]
pub type AutoCmd12ErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ADMA_ERR` reader - "]
pub type AdmaErrR = crate::BitReader;
#[doc = "Field `ADMA_ERR` writer - "]
pub type AdmaErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TUNE_ERR` reader - "]
pub type TuneErrR = crate::BitReader;
#[doc = "Field `TUNE_ERR` writer - "]
pub type TuneErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SPI_ERR` reader - "]
pub type SpiErrR = crate::BitReader;
#[doc = "Field `SPI_ERR` writer - "]
pub type SpiErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AXI_RESP_ERR` reader - "]
pub type AxiRespErrR = crate::BitReader;
#[doc = "Field `AXI_RESP_ERR` writer - "]
pub type AxiRespErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CPL_TIMEOUT_ERR` reader - "]
pub type CplTimeoutErrR = crate::BitReader;
#[doc = "Field `CPL_TIMEOUT_ERR` writer - "]
pub type CplTimeoutErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CRC_STATUS_ERR` reader - "]
pub type CrcStatusErrR = crate::BitReader;
#[doc = "Field `CRC_STATUS_ERR` writer - "]
pub type CrcStatusErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmd_timeout_err(&self) -> CmdTimeoutErrR {
        CmdTimeoutErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cmd_crc_err(&self) -> CmdCrcErrR {
        CmdCrcErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cmd_end_bit_err(&self) -> CmdEndBitErrR {
        CmdEndBitErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cmd_index_err(&self) -> CmdIndexErrR {
        CmdIndexErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn data_timeout_err(&self) -> DataTimeoutErrR {
        DataTimeoutErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rd_data_crc_err(&self) -> RdDataCrcErrR {
        RdDataCrcErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rd_data_end_bit_err(&self) -> RdDataEndBitErrR {
        RdDataEndBitErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cur_limit_err(&self) -> CurLimitErrR {
        CurLimitErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn auto_cmd12_err(&self) -> AutoCmd12ErrR {
        AutoCmd12ErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn adma_err(&self) -> AdmaErrR {
        AdmaErrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tune_err(&self) -> TuneErrR {
        TuneErrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi_err(&self) -> SpiErrR {
        SpiErrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn axi_resp_err(&self) -> AxiRespErrR {
        AxiRespErrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cpl_timeout_err(&self) -> CplTimeoutErrR {
        CplTimeoutErrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn crc_status_err(&self) -> CrcStatusErrR {
        CrcStatusErrR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmd_timeout_err(&mut self) -> CmdTimeoutErrW<'_, SdErrorIntStatusSpec> {
        CmdTimeoutErrW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cmd_crc_err(&mut self) -> CmdCrcErrW<'_, SdErrorIntStatusSpec> {
        CmdCrcErrW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cmd_end_bit_err(&mut self) -> CmdEndBitErrW<'_, SdErrorIntStatusSpec> {
        CmdEndBitErrW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cmd_index_err(&mut self) -> CmdIndexErrW<'_, SdErrorIntStatusSpec> {
        CmdIndexErrW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn data_timeout_err(&mut self) -> DataTimeoutErrW<'_, SdErrorIntStatusSpec> {
        DataTimeoutErrW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rd_data_crc_err(&mut self) -> RdDataCrcErrW<'_, SdErrorIntStatusSpec> {
        RdDataCrcErrW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rd_data_end_bit_err(&mut self) -> RdDataEndBitErrW<'_, SdErrorIntStatusSpec> {
        RdDataEndBitErrW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cur_limit_err(&mut self) -> CurLimitErrW<'_, SdErrorIntStatusSpec> {
        CurLimitErrW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn auto_cmd12_err(&mut self) -> AutoCmd12ErrW<'_, SdErrorIntStatusSpec> {
        AutoCmd12ErrW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn adma_err(&mut self) -> AdmaErrW<'_, SdErrorIntStatusSpec> {
        AdmaErrW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tune_err(&mut self) -> TuneErrW<'_, SdErrorIntStatusSpec> {
        TuneErrW::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi_err(&mut self) -> SpiErrW<'_, SdErrorIntStatusSpec> {
        SpiErrW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn axi_resp_err(&mut self) -> AxiRespErrW<'_, SdErrorIntStatusSpec> {
        AxiRespErrW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cpl_timeout_err(&mut self) -> CplTimeoutErrW<'_, SdErrorIntStatusSpec> {
        CplTimeoutErrW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn crc_status_err(&mut self) -> CrcStatusErrW<'_, SdErrorIntStatusSpec> {
        CrcStatusErrW::new(self, 15)
    }
}
#[doc = "Error Interrupt Status Register (Read/Write 1 to Clear)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_error_int_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_error_int_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdErrorIntStatusSpec;
impl crate::RegisterSpec for SdErrorIntStatusSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_error_int_status::R`](R) reader structure"]
impl crate::Readable for SdErrorIntStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_error_int_status::W`](W) writer structure"]
impl crate::Writable for SdErrorIntStatusSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0xf7ff;
}
#[doc = "`reset()` method sets SD_ERROR_INT_STATUS to value 0"]
impl crate::Resettable for SdErrorIntStatusSpec {}
