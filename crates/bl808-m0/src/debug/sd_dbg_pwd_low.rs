#[doc = "Register `SD_DBG_PWD_LOW` reader"]
pub type R = crate::R<SdDbgPwdLowSpec>;
#[doc = "Register `SD_DBG_PWD_LOW` writer"]
pub type W = crate::W<SdDbgPwdLowSpec>;
#[doc = "Field `sd_dbg_pwd_low` reader - "]
pub type SdDbgPwdLowR = crate::FieldReader<u32>;
#[doc = "Field `sd_dbg_pwd_low` writer - "]
pub type SdDbgPwdLowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_dbg_pwd_low(&self) -> SdDbgPwdLowR {
        SdDbgPwdLowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_dbg_pwd_low(&mut self) -> SdDbgPwdLowW<'_, SdDbgPwdLowSpec> {
        SdDbgPwdLowW::new(self, 0)
    }
}
#[doc = "Debug Access Password 1 Low Word\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_dbg_pwd_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_dbg_pwd_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdDbgPwdLowSpec;
impl crate::RegisterSpec for SdDbgPwdLowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sd_dbg_pwd_low::R`](R) reader structure"]
impl crate::Readable for SdDbgPwdLowSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_dbg_pwd_low::W`](W) writer structure"]
impl crate::Writable for SdDbgPwdLowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_DBG_PWD_LOW to value 0"]
impl crate::Resettable for SdDbgPwdLowSpec {}
