#[doc = "Register `DBI_FIFO_WDATA` writer"]
pub type W = crate::W<DbiFifoWdataSpec>;
#[doc = "Field `dbi_fifo_wdata` writer - Write data to the DBI FIFO"]
pub type DbiFifoWdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Write data to the DBI FIFO"]
    #[inline(always)]
    pub fn dbi_fifo_wdata(&mut self) -> DbiFifoWdataW<'_, DbiFifoWdataSpec> {
        DbiFifoWdataW::new(self, 0)
    }
}
#[doc = "DBI FIFO Write Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbi_fifo_wdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbiFifoWdataSpec;
impl crate::RegisterSpec for DbiFifoWdataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dbi_fifo_wdata::W`](W) writer structure"]
impl crate::Writable for DbiFifoWdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBI_FIFO_WDATA to value 0"]
impl crate::Resettable for DbiFifoWdataSpec {}
