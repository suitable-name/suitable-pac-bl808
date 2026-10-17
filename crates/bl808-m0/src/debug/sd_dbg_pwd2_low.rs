#[doc = "Register `SD_DBG_PWD2_LOW` reader"]
pub type R = crate::R<SdDbgPwd2LowSpec>;
#[doc = "Register `SD_DBG_PWD2_LOW` writer"]
pub type W = crate::W<SdDbgPwd2LowSpec>;
#[doc = "Field `sd_dbg_pwd2_low` reader - "]
pub type SdDbgPwd2LowR = crate::FieldReader<u32>;
#[doc = "Field `sd_dbg_pwd2_low` writer - "]
pub type SdDbgPwd2LowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_dbg_pwd2_low(&self) -> SdDbgPwd2LowR {
        SdDbgPwd2LowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_dbg_pwd2_low(&mut self) -> SdDbgPwd2LowW<'_, SdDbgPwd2LowSpec> {
        SdDbgPwd2LowW::new(self, 0)
    }
}
#[doc = "Debug Access Password 2 Low Word (Write to this triggers authentication)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_dbg_pwd2_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_dbg_pwd2_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdDbgPwd2LowSpec;
impl crate::RegisterSpec for SdDbgPwd2LowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sd_dbg_pwd2_low::R`](R) reader structure"]
impl crate::Readable for SdDbgPwd2LowSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_dbg_pwd2_low::W`](W) writer structure"]
impl crate::Writable for SdDbgPwd2LowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_DBG_PWD2_LOW to value 0"]
impl crate::Resettable for SdDbgPwd2LowSpec {}
