#[doc = "Register `SD_FORCE_EVENT_FOR_ERROR_STATUS` writer"]
pub type W = crate::W<SdForceEventForErrorStatusSpec>;
#[doc = "Field `F_CMD_TO_ERR` writer - "]
pub type FCmdToErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F_CMD_CRC_ERR` writer - "]
pub type FCmdCrcErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F_CMD_END_BIT_ERR` writer - "]
pub type FCmdEndBitErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F_CMD_INDEX_ERR` writer - "]
pub type FCmdIndexErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F_DAT_TO_ERR` writer - "]
pub type FDatToErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F_DAT_CRC_ERR` writer - "]
pub type FDatCrcErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F_DAT_END_BIT_ERR` writer - "]
pub type FDatEndBitErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F_CURRENT_ERR` writer - "]
pub type FCurrentErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F_ACMD12_ERR` writer - "]
pub type FAcmd12ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F_ADMA_ERR` writer - "]
pub type FAdmaErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F_SPI_ERR` writer - "]
pub type FSpiErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F_AXI_RESP_ERR` writer - "]
pub type FAxiRespErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F_CPL_TIMEOUT_ERR` writer - "]
pub type FCplTimeoutErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F_CRC_STATUS_ERR` writer - "]
pub type FCrcStatusErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn f_cmd_to_err(&mut self) -> FCmdToErrW<'_, SdForceEventForErrorStatusSpec> {
        FCmdToErrW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn f_cmd_crc_err(&mut self) -> FCmdCrcErrW<'_, SdForceEventForErrorStatusSpec> {
        FCmdCrcErrW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn f_cmd_end_bit_err(&mut self) -> FCmdEndBitErrW<'_, SdForceEventForErrorStatusSpec> {
        FCmdEndBitErrW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn f_cmd_index_err(&mut self) -> FCmdIndexErrW<'_, SdForceEventForErrorStatusSpec> {
        FCmdIndexErrW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn f_dat_to_err(&mut self) -> FDatToErrW<'_, SdForceEventForErrorStatusSpec> {
        FDatToErrW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn f_dat_crc_err(&mut self) -> FDatCrcErrW<'_, SdForceEventForErrorStatusSpec> {
        FDatCrcErrW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn f_dat_end_bit_err(&mut self) -> FDatEndBitErrW<'_, SdForceEventForErrorStatusSpec> {
        FDatEndBitErrW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn f_current_err(&mut self) -> FCurrentErrW<'_, SdForceEventForErrorStatusSpec> {
        FCurrentErrW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn f_acmd12_err(&mut self) -> FAcmd12ErrW<'_, SdForceEventForErrorStatusSpec> {
        FAcmd12ErrW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn f_adma_err(&mut self) -> FAdmaErrW<'_, SdForceEventForErrorStatusSpec> {
        FAdmaErrW::new(self, 9)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn f_spi_err(&mut self) -> FSpiErrW<'_, SdForceEventForErrorStatusSpec> {
        FSpiErrW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn f_axi_resp_err(&mut self) -> FAxiRespErrW<'_, SdForceEventForErrorStatusSpec> {
        FAxiRespErrW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn f_cpl_timeout_err(&mut self) -> FCplTimeoutErrW<'_, SdForceEventForErrorStatusSpec> {
        FCplTimeoutErrW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn f_crc_status_err(&mut self) -> FCrcStatusErrW<'_, SdForceEventForErrorStatusSpec> {
        FCrcStatusErrW::new(self, 15)
    }
}
#[doc = "Force Event Register for Error Status\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_force_event_for_error_status::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdForceEventForErrorStatusSpec;
impl crate::RegisterSpec for SdForceEventForErrorStatusSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`sd_force_event_for_error_status::W`](W) writer structure"]
impl crate::Writable for SdForceEventForErrorStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_FORCE_EVENT_FOR_ERROR_STATUS to value 0"]
impl crate::Resettable for SdForceEventForErrorStatusSpec {}
