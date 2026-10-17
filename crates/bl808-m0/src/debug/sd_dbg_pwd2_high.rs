#[doc = "Register `SD_DBG_PWD2_HIGH` reader"]
pub type R = crate::R<SdDbgPwd2HighSpec>;
#[doc = "Register `SD_DBG_PWD2_HIGH` writer"]
pub type W = crate::W<SdDbgPwd2HighSpec>;
#[doc = "Field `sd_dbg_pwd2_high` reader - "]
pub type SdDbgPwd2HighR = crate::FieldReader<u32>;
#[doc = "Field `sd_dbg_pwd2_high` writer - "]
pub type SdDbgPwd2HighW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_dbg_pwd2_high(&self) -> SdDbgPwd2HighR {
        SdDbgPwd2HighR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_dbg_pwd2_high(&mut self) -> SdDbgPwd2HighW<'_, SdDbgPwd2HighSpec> {
        SdDbgPwd2HighW::new(self, 0)
    }
}
#[doc = "Debug Access Password 2 High Word\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_dbg_pwd2_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_dbg_pwd2_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdDbgPwd2HighSpec;
impl crate::RegisterSpec for SdDbgPwd2HighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sd_dbg_pwd2_high::R`](R) reader structure"]
impl crate::Readable for SdDbgPwd2HighSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_dbg_pwd2_high::W`](W) writer structure"]
impl crate::Writable for SdDbgPwd2HighSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_DBG_PWD2_HIGH to value 0"]
impl crate::Resettable for SdDbgPwd2HighSpec {}
