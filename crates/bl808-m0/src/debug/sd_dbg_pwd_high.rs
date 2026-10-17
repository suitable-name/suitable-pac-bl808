#[doc = "Register `SD_DBG_PWD_HIGH` reader"]
pub type R = crate::R<SdDbgPwdHighSpec>;
#[doc = "Register `SD_DBG_PWD_HIGH` writer"]
pub type W = crate::W<SdDbgPwdHighSpec>;
#[doc = "Field `sd_dbg_pwd_high` reader - "]
pub type SdDbgPwdHighR = crate::FieldReader<u32>;
#[doc = "Field `sd_dbg_pwd_high` writer - "]
pub type SdDbgPwdHighW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_dbg_pwd_high(&self) -> SdDbgPwdHighR {
        SdDbgPwdHighR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_dbg_pwd_high(&mut self) -> SdDbgPwdHighW<'_, SdDbgPwdHighSpec> {
        SdDbgPwdHighW::new(self, 0)
    }
}
#[doc = "Debug Access Password 1 High Word\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_dbg_pwd_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_dbg_pwd_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdDbgPwdHighSpec;
impl crate::RegisterSpec for SdDbgPwdHighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sd_dbg_pwd_high::R`](R) reader structure"]
impl crate::Readable for SdDbgPwdHighSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_dbg_pwd_high::W`](W) writer structure"]
impl crate::Writable for SdDbgPwdHighSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_DBG_PWD_HIGH to value 0"]
impl crate::Resettable for SdDbgPwdHighSpec {}
