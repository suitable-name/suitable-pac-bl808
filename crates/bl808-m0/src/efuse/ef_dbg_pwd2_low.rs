#[doc = "Register `EF_DBG_PWD2_LOW` reader"]
pub type R = crate::R<EfDbgPwd2LowSpec>;
#[doc = "Register `EF_DBG_PWD2_LOW` writer"]
pub type W = crate::W<EfDbgPwd2LowSpec>;
#[doc = "Field `ef_dbg_pwd2_low` reader - "]
pub type EfDbgPwd2LowR = crate::FieldReader<u32>;
#[doc = "Field `ef_dbg_pwd2_low` writer - "]
pub type EfDbgPwd2LowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_dbg_pwd2_low(&self) -> EfDbgPwd2LowR {
        EfDbgPwd2LowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_dbg_pwd2_low(&mut self) -> EfDbgPwd2LowW<'_, EfDbgPwd2LowSpec> {
        EfDbgPwd2LowW::new(self, 0)
    }
}
#[doc = "Debug Password 2 Low Word\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_dbg_pwd2_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_dbg_pwd2_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfDbgPwd2LowSpec;
impl crate::RegisterSpec for EfDbgPwd2LowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_dbg_pwd2_low::R`](R) reader structure"]
impl crate::Readable for EfDbgPwd2LowSpec {}
#[doc = "`write(|w| ..)` method takes [`ef_dbg_pwd2_low::W`](W) writer structure"]
impl crate::Writable for EfDbgPwd2LowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_DBG_PWD2_LOW to value 0"]
impl crate::Resettable for EfDbgPwd2LowSpec {}
