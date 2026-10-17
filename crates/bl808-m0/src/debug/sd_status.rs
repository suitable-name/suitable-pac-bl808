#[doc = "Register `SD_STATUS` reader"]
pub type R = crate::R<SdStatusSpec>;
#[doc = "Register `SD_STATUS` writer"]
pub type W = crate::W<SdStatusSpec>;
#[doc = "Field `sd_dbg_pwd_busy` reader - Debug password verification busy status"]
pub type SdDbgPwdBusyR = crate::BitReader;
#[doc = "Field `sd_dbg_pwd_trig` reader - Debug password write trigger (must be set high for PWD check)"]
pub type SdDbgPwdTrigR = crate::BitReader;
#[doc = "Field `sd_dbg_pwd_trig` writer - Debug password write trigger (must be set high for PWD check)"]
pub type SdDbgPwdTrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sd_dbg_cci_read_en` reader - CCI read enable for debug"]
pub type SdDbgCciReadEnR = crate::BitReader;
#[doc = "Field `sd_dbg_cci_read_en` writer - CCI read enable for debug"]
pub type SdDbgCciReadEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sd_dbg_cci_clk_sel` reader - CCI clock select for debug"]
pub type SdDbgCciClkSelR = crate::BitReader;
#[doc = "Field `sd_dbg_cci_clk_sel` writer - CCI clock select for debug"]
pub type SdDbgCciClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sd_dbg_pwd_cnt` reader - Debug password timer/counter"]
pub type SdDbgPwdCntR = crate::FieldReader<u32>;
#[doc = "Field `sd_dbg_mode` reader - Debug mode status"]
pub type SdDbgModeR = crate::FieldReader;
#[doc = "Field `sd_dbg_ena` reader - Debug interface enable status"]
pub type SdDbgEnaR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Debug password verification busy status"]
    #[inline(always)]
    pub fn sd_dbg_pwd_busy(&self) -> SdDbgPwdBusyR {
        SdDbgPwdBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug password write trigger (must be set high for PWD check)"]
    #[inline(always)]
    pub fn sd_dbg_pwd_trig(&self) -> SdDbgPwdTrigR {
        SdDbgPwdTrigR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CCI read enable for debug"]
    #[inline(always)]
    pub fn sd_dbg_cci_read_en(&self) -> SdDbgCciReadEnR {
        SdDbgCciReadEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CCI clock select for debug"]
    #[inline(always)]
    pub fn sd_dbg_cci_clk_sel(&self) -> SdDbgCciClkSelR {
        SdDbgCciClkSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:23 - Debug password timer/counter"]
    #[inline(always)]
    pub fn sd_dbg_pwd_cnt(&self) -> SdDbgPwdCntR {
        SdDbgPwdCntR::new((self.bits >> 4) & 0x000f_ffff)
    }
    #[doc = "Bits 24:27 - Debug mode status"]
    #[inline(always)]
    pub fn sd_dbg_mode(&self) -> SdDbgModeR {
        SdDbgModeR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Debug interface enable status"]
    #[inline(always)]
    pub fn sd_dbg_ena(&self) -> SdDbgEnaR {
        SdDbgEnaR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Debug password write trigger (must be set high for PWD check)"]
    #[inline(always)]
    pub fn sd_dbg_pwd_trig(&mut self) -> SdDbgPwdTrigW<'_, SdStatusSpec> {
        SdDbgPwdTrigW::new(self, 1)
    }
    #[doc = "Bit 2 - CCI read enable for debug"]
    #[inline(always)]
    pub fn sd_dbg_cci_read_en(&mut self) -> SdDbgCciReadEnW<'_, SdStatusSpec> {
        SdDbgCciReadEnW::new(self, 2)
    }
    #[doc = "Bit 3 - CCI clock select for debug"]
    #[inline(always)]
    pub fn sd_dbg_cci_clk_sel(&mut self) -> SdDbgCciClkSelW<'_, SdStatusSpec> {
        SdDbgCciClkSelW::new(self, 3)
    }
}
#[doc = "Debug Status and Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdStatusSpec;
impl crate::RegisterSpec for SdStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sd_status::R`](R) reader structure"]
impl crate::Readable for SdStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_status::W`](W) writer structure"]
impl crate::Writable for SdStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_STATUS to value 0"]
impl crate::Resettable for SdStatusSpec {}
