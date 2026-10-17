#[doc = "Register `EF_DBG_PWD_HIGH` reader"]
pub type R = crate::R<EfDbgPwdHighSpec>;
#[doc = "Register `EF_DBG_PWD_HIGH` writer"]
pub type W = crate::W<EfDbgPwdHighSpec>;
#[doc = "Field `ef_dbg_pwd_high` reader - "]
pub type EfDbgPwdHighR = crate::FieldReader<u32>;
#[doc = "Field `ef_dbg_pwd_high` writer - "]
pub type EfDbgPwdHighW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_dbg_pwd_high(&self) -> EfDbgPwdHighR {
        EfDbgPwdHighR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_dbg_pwd_high(&mut self) -> EfDbgPwdHighW<'_, EfDbgPwdHighSpec> {
        EfDbgPwdHighW::new(self, 0)
    }
}
#[doc = "Debug Password High Word\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_dbg_pwd_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_dbg_pwd_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfDbgPwdHighSpec;
impl crate::RegisterSpec for EfDbgPwdHighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_dbg_pwd_high::R`](R) reader structure"]
impl crate::Readable for EfDbgPwdHighSpec {}
#[doc = "`write(|w| ..)` method takes [`ef_dbg_pwd_high::W`](W) writer structure"]
impl crate::Writable for EfDbgPwdHighSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_DBG_PWD_HIGH to value 0"]
impl crate::Resettable for EfDbgPwdHighSpec {}
