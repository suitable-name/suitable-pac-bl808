#[doc = "Register `SD_AUTO_CMD12_ERROR_STATUS` reader"]
pub type R = crate::R<SdAutoCmd12ErrorStatusSpec>;
#[doc = "Register `SD_AUTO_CMD12_ERROR_STATUS` writer"]
pub type W = crate::W<SdAutoCmd12ErrorStatusSpec>;
#[doc = "Field `AUTO_CMD12_NOT_EXE` reader - "]
pub type AutoCmd12NotExeR = crate::BitReader;
#[doc = "Field `AUTO_CMD12_NOT_EXE` writer - "]
pub type AutoCmd12NotExeW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AUTO_CMD_TIMEOUT_ERR` reader - "]
pub type AutoCmdTimeoutErrR = crate::BitReader;
#[doc = "Field `AUTO_CMD_TIMEOUT_ERR` writer - "]
pub type AutoCmdTimeoutErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AUTO_CMD_CRC_ERR` reader - "]
pub type AutoCmdCrcErrR = crate::BitReader;
#[doc = "Field `AUTO_CMD_CRC_ERR` writer - "]
pub type AutoCmdCrcErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AUTO_CMD_END_BIT_ERR` reader - "]
pub type AutoCmdEndBitErrR = crate::BitReader;
#[doc = "Field `AUTO_CMD_END_BIT_ERR` writer - "]
pub type AutoCmdEndBitErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AUTO_CMD_INDEX_ERR` reader - "]
pub type AutoCmdIndexErrR = crate::BitReader;
#[doc = "Field `AUTO_CMD_INDEX_ERR` writer - "]
pub type AutoCmdIndexErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CMD_NOT_ISSUED` reader - "]
pub type CmdNotIssuedR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn auto_cmd12_not_exe(&self) -> AutoCmd12NotExeR {
        AutoCmd12NotExeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn auto_cmd_timeout_err(&self) -> AutoCmdTimeoutErrR {
        AutoCmdTimeoutErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn auto_cmd_crc_err(&self) -> AutoCmdCrcErrR {
        AutoCmdCrcErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn auto_cmd_end_bit_err(&self) -> AutoCmdEndBitErrR {
        AutoCmdEndBitErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn auto_cmd_index_err(&self) -> AutoCmdIndexErrR {
        AutoCmdIndexErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cmd_not_issued(&self) -> CmdNotIssuedR {
        CmdNotIssuedR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn auto_cmd12_not_exe(&mut self) -> AutoCmd12NotExeW<'_, SdAutoCmd12ErrorStatusSpec> {
        AutoCmd12NotExeW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn auto_cmd_timeout_err(&mut self) -> AutoCmdTimeoutErrW<'_, SdAutoCmd12ErrorStatusSpec> {
        AutoCmdTimeoutErrW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn auto_cmd_crc_err(&mut self) -> AutoCmdCrcErrW<'_, SdAutoCmd12ErrorStatusSpec> {
        AutoCmdCrcErrW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn auto_cmd_end_bit_err(&mut self) -> AutoCmdEndBitErrW<'_, SdAutoCmd12ErrorStatusSpec> {
        AutoCmdEndBitErrW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn auto_cmd_index_err(&mut self) -> AutoCmdIndexErrW<'_, SdAutoCmd12ErrorStatusSpec> {
        AutoCmdIndexErrW::new(self, 4)
    }
}
#[doc = "Auto CMD12/23 Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_auto_cmd12_error_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_auto_cmd12_error_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdAutoCmd12ErrorStatusSpec;
impl crate::RegisterSpec for SdAutoCmd12ErrorStatusSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_auto_cmd12_error_status::R`](R) reader structure"]
impl crate::Readable for SdAutoCmd12ErrorStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_auto_cmd12_error_status::W`](W) writer structure"]
impl crate::Writable for SdAutoCmd12ErrorStatusSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0x1f;
}
#[doc = "`reset()` method sets SD_AUTO_CMD12_ERROR_STATUS to value 0"]
impl crate::Resettable for SdAutoCmd12ErrorStatusSpec {}
