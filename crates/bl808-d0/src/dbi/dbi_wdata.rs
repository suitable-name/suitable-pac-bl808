#[doc = "Register `DBI_WDATA` reader"]
pub type R = crate::R<DbiWdataSpec>;
#[doc = "Register `DBI_WDATA` writer"]
pub type W = crate::W<DbiWdataSpec>;
#[doc = "Field `cr_dbi_wdata` reader - Manual write data or read back of last written command."]
pub type CrDbiWdataR = crate::FieldReader<u32>;
#[doc = "Field `cr_dbi_wdata` writer - Manual write data or read back of last written command."]
pub type CrDbiWdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Manual write data or read back of last written command."]
    #[inline(always)]
    pub fn cr_dbi_wdata(&self) -> CrDbiWdataR {
        CrDbiWdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Manual write data or read back of last written command."]
    #[inline(always)]
    pub fn cr_dbi_wdata(&mut self) -> CrDbiWdataW<'_, DbiWdataSpec> {
        CrDbiWdataW::new(self, 0)
    }
}
#[doc = "DBI Write Data Register (Manual/FIFO Bypass)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbi_wdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbi_wdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbiWdataSpec;
impl crate::RegisterSpec for DbiWdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbi_wdata::R`](R) reader structure"]
impl crate::Readable for DbiWdataSpec {}
#[doc = "`write(|w| ..)` method takes [`dbi_wdata::W`](W) writer structure"]
impl crate::Writable for DbiWdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBI_WDATA to value 0"]
impl crate::Resettable for DbiWdataSpec {}
