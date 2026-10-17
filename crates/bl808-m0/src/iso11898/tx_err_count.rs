#[doc = "Register `TX_ERR_COUNT` reader"]
pub type R = crate::R<TxErrCountSpec>;
#[doc = "Register `TX_ERR_COUNT` writer"]
pub type W = crate::W<TxErrCountSpec>;
#[doc = "Field `tx_err_cnt` reader - 8-bit TEC value"]
pub type TxErrCntR = crate::FieldReader;
#[doc = "Field `tx_err_cnt` writer - 8-bit TEC value"]
pub type TxErrCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit TEC value"]
    #[inline(always)]
    pub fn tx_err_cnt(&self) -> TxErrCntR {
        TxErrCntR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit TEC value"]
    #[inline(always)]
    pub fn tx_err_cnt(&mut self) -> TxErrCntW<'_, TxErrCountSpec> {
        TxErrCntW::new(self, 0)
    }
}
#[doc = "Transmit Error Counter Register. Reflects the current value of the CAN transmit error counter (TEC).\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_err_count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_err_count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxErrCountSpec;
impl crate::RegisterSpec for TxErrCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_err_count::R`](R) reader structure"]
impl crate::Readable for TxErrCountSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_err_count::W`](W) writer structure"]
impl crate::Writable for TxErrCountSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_ERR_COUNT to value 0"]
impl crate::Resettable for TxErrCountSpec {}
