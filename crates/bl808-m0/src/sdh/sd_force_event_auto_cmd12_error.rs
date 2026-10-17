#[doc = "Register `SD_FORCE_EVENT_AUTO_CMD12_ERROR` writer"]
pub type W = crate::W<SdForceEventAutoCmd12ErrorSpec>;
#[doc = "Field `F_ACMD12_NEXE_ERR` writer - "]
pub type FAcmd12NexeErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F_ACMD_TO_ERR` writer - "]
pub type FAcmdToErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F_ACMD_CRC_ERR` writer - "]
pub type FAcmdCrcErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F__ACMD_EBIT_ERR` writer - "]
pub type F_AcmdEbitErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F_ACMD_INDEX_ERR` writer - "]
pub type FAcmdIndexErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F_ACMD12_ISSUE_ERR` writer - "]
pub type FAcmd12IssueErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn f_acmd12_nexe_err(&mut self) -> FAcmd12NexeErrW<'_, SdForceEventAutoCmd12ErrorSpec> {
        FAcmd12NexeErrW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn f_acmd_to_err(&mut self) -> FAcmdToErrW<'_, SdForceEventAutoCmd12ErrorSpec> {
        FAcmdToErrW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn f_acmd_crc_err(&mut self) -> FAcmdCrcErrW<'_, SdForceEventAutoCmd12ErrorSpec> {
        FAcmdCrcErrW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn f__acmd_ebit_err(&mut self) -> F_AcmdEbitErrW<'_, SdForceEventAutoCmd12ErrorSpec> {
        F_AcmdEbitErrW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn f_acmd_index_err(&mut self) -> FAcmdIndexErrW<'_, SdForceEventAutoCmd12ErrorSpec> {
        FAcmdIndexErrW::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn f_acmd12_issue_err(&mut self) -> FAcmd12IssueErrW<'_, SdForceEventAutoCmd12ErrorSpec> {
        FAcmd12IssueErrW::new(self, 7)
    }
}
#[doc = "Force Event Register for Auto CMD12 Error Status\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_force_event_auto_cmd12_error::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdForceEventAutoCmd12ErrorSpec;
impl crate::RegisterSpec for SdForceEventAutoCmd12ErrorSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`sd_force_event_auto_cmd12_error::W`](W) writer structure"]
impl crate::Writable for SdForceEventAutoCmd12ErrorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_FORCE_EVENT_AUTO_CMD12_ERROR to value 0"]
impl crate::Resettable for SdForceEventAutoCmd12ErrorSpec {}
