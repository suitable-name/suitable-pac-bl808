#[doc = "Register `DSI_FIFO_WDATA` writer"]
pub type W = crate::W<DsiFifoWdataSpec>;
#[doc = "Field `dsi_fifo_wdata` writer - "]
pub type DsiFifoWdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dsi_fifo_wdata(&mut self) -> DsiFifoWdataW<'_, DsiFifoWdataSpec> {
        DsiFifoWdataW::new(self, 0)
    }
}
#[doc = "FIFO Write Data\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_fifo_wdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiFifoWdataSpec;
impl crate::RegisterSpec for DsiFifoWdataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dsi_fifo_wdata::W`](W) writer structure"]
impl crate::Writable for DsiFifoWdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_FIFO_WDATA to value 0"]
impl crate::Resettable for DsiFifoWdataSpec {}
