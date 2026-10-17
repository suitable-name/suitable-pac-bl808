#[doc = "Register `SD_CMD` reader"]
pub type R = crate::R<SdCmdSpec>;
#[doc = "Register `SD_CMD` writer"]
pub type W = crate::W<SdCmdSpec>;
#[doc = "Field `RESP_TYPE` reader - Response Type Select"]
pub type RespTypeR = crate::FieldReader;
#[doc = "Field `RESP_TYPE` writer - Response Type Select"]
pub type RespTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMD_CRC_CHK_EN` reader - Command CRC Check Enable"]
pub type CmdCrcChkEnR = crate::BitReader;
#[doc = "Field `CMD_CRC_CHK_EN` writer - Command CRC Check Enable"]
pub type CmdCrcChkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_INDEX_CHK_EN` reader - Command Index Check Enable"]
pub type CmdIndexChkEnR = crate::BitReader;
#[doc = "Field `CMD_INDEX_CHK_EN` writer - Command Index Check Enable"]
pub type CmdIndexChkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_PRESENT` reader - Data Present Select"]
pub type DataPresentR = crate::BitReader;
#[doc = "Field `DATA_PRESENT` writer - Data Present Select"]
pub type DataPresentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_TYPE` reader - Command Type"]
pub type CmdTypeR = crate::FieldReader;
#[doc = "Field `CMD_TYPE` writer - Command Type"]
pub type CmdTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMD_INDEX` reader - Command Index (0-63)"]
pub type CmdIndexR = crate::FieldReader;
#[doc = "Field `CMD_INDEX` writer - Command Index (0-63)"]
pub type CmdIndexW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - Response Type Select"]
    #[inline(always)]
    pub fn resp_type(&self) -> RespTypeR {
        RespTypeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Command CRC Check Enable"]
    #[inline(always)]
    pub fn cmd_crc_chk_en(&self) -> CmdCrcChkEnR {
        CmdCrcChkEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Command Index Check Enable"]
    #[inline(always)]
    pub fn cmd_index_chk_en(&self) -> CmdIndexChkEnR {
        CmdIndexChkEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Present Select"]
    #[inline(always)]
    pub fn data_present(&self) -> DataPresentR {
        DataPresentR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Command Type"]
    #[inline(always)]
    pub fn cmd_type(&self) -> CmdTypeR {
        CmdTypeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:13 - Command Index (0-63)"]
    #[inline(always)]
    pub fn cmd_index(&self) -> CmdIndexR {
        CmdIndexR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Response Type Select"]
    #[inline(always)]
    pub fn resp_type(&mut self) -> RespTypeW<'_, SdCmdSpec> {
        RespTypeW::new(self, 0)
    }
    #[doc = "Bit 3 - Command CRC Check Enable"]
    #[inline(always)]
    pub fn cmd_crc_chk_en(&mut self) -> CmdCrcChkEnW<'_, SdCmdSpec> {
        CmdCrcChkEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Command Index Check Enable"]
    #[inline(always)]
    pub fn cmd_index_chk_en(&mut self) -> CmdIndexChkEnW<'_, SdCmdSpec> {
        CmdIndexChkEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Data Present Select"]
    #[inline(always)]
    pub fn data_present(&mut self) -> DataPresentW<'_, SdCmdSpec> {
        DataPresentW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Command Type"]
    #[inline(always)]
    pub fn cmd_type(&mut self) -> CmdTypeW<'_, SdCmdSpec> {
        CmdTypeW::new(self, 6)
    }
    #[doc = "Bits 8:13 - Command Index (0-63)"]
    #[inline(always)]
    pub fn cmd_index(&mut self) -> CmdIndexW<'_, SdCmdSpec> {
        CmdIndexW::new(self, 8)
    }
}
#[doc = "Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdCmdSpec;
impl crate::RegisterSpec for SdCmdSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_cmd::R`](R) reader structure"]
impl crate::Readable for SdCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_cmd::W`](W) writer structure"]
impl crate::Writable for SdCmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_CMD to value 0"]
impl crate::Resettable for SdCmdSpec {}
