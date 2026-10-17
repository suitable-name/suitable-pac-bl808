#[doc = "Register `EF_DBG_PWD2_HIGH` reader"]
pub type R = crate::R<EfDbgPwd2HighSpec>;
#[doc = "Register `EF_DBG_PWD2_HIGH` writer"]
pub type W = crate::W<EfDbgPwd2HighSpec>;
#[doc = "Field `ef_dbg_pwd2_high` reader - "]
pub type EfDbgPwd2HighR = crate::FieldReader<u32>;
#[doc = "Field `ef_dbg_pwd2_high` writer - "]
pub type EfDbgPwd2HighW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_dbg_pwd2_high(&self) -> EfDbgPwd2HighR {
        EfDbgPwd2HighR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_dbg_pwd2_high(&mut self) -> EfDbgPwd2HighW<'_, EfDbgPwd2HighSpec> {
        EfDbgPwd2HighW::new(self, 0)
    }
}
#[doc = "Debug Password 2 High Word\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_dbg_pwd2_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_dbg_pwd2_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfDbgPwd2HighSpec;
impl crate::RegisterSpec for EfDbgPwd2HighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_dbg_pwd2_high::R`](R) reader structure"]
impl crate::Readable for EfDbgPwd2HighSpec {}
#[doc = "`write(|w| ..)` method takes [`ef_dbg_pwd2_high::W`](W) writer structure"]
impl crate::Writable for EfDbgPwd2HighSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_DBG_PWD2_HIGH to value 0"]
impl crate::Resettable for EfDbgPwd2HighSpec {}
