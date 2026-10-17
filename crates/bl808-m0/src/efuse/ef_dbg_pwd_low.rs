#[doc = "Register `EF_DBG_PWD_LOW` reader"]
pub type R = crate::R<EfDbgPwdLowSpec>;
#[doc = "Register `EF_DBG_PWD_LOW` writer"]
pub type W = crate::W<EfDbgPwdLowSpec>;
#[doc = "Field `ef_dbg_pwd_low` reader - "]
pub type EfDbgPwdLowR = crate::FieldReader<u32>;
#[doc = "Field `ef_dbg_pwd_low` writer - "]
pub type EfDbgPwdLowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_dbg_pwd_low(&self) -> EfDbgPwdLowR {
        EfDbgPwdLowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_dbg_pwd_low(&mut self) -> EfDbgPwdLowW<'_, EfDbgPwdLowSpec> {
        EfDbgPwdLowW::new(self, 0)
    }
}
#[doc = "Debug Password Low Word\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_dbg_pwd_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_dbg_pwd_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfDbgPwdLowSpec;
impl crate::RegisterSpec for EfDbgPwdLowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_dbg_pwd_low::R`](R) reader structure"]
impl crate::Readable for EfDbgPwdLowSpec {}
#[doc = "`write(|w| ..)` method takes [`ef_dbg_pwd_low::W`](W) writer structure"]
impl crate::Writable for EfDbgPwdLowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_DBG_PWD_LOW to value 0"]
impl crate::Resettable for EfDbgPwdLowSpec {}
