#[doc = "Register `RX_ERR_COUNT` reader"]
pub type R = crate::R<RxErrCountSpec>;
#[doc = "Register `RX_ERR_COUNT` writer"]
pub type W = crate::W<RxErrCountSpec>;
#[doc = "Field `rx_err_cnt` reader - 8-bit REC value"]
pub type RxErrCntR = crate::FieldReader;
#[doc = "Field `rx_err_cnt` writer - 8-bit REC value"]
pub type RxErrCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit REC value"]
    #[inline(always)]
    pub fn rx_err_cnt(&self) -> RxErrCntR {
        RxErrCntR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit REC value"]
    #[inline(always)]
    pub fn rx_err_cnt(&mut self) -> RxErrCntW<'_, RxErrCountSpec> {
        RxErrCntW::new(self, 0)
    }
}
#[doc = "Receive Error Counter Register. Reflects the current value of the CAN receive error counter (REC).\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_err_count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_err_count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxErrCountSpec;
impl crate::RegisterSpec for RxErrCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_err_count::R`](R) reader structure"]
impl crate::Readable for RxErrCountSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_err_count::W`](W) writer structure"]
impl crate::Writable for RxErrCountSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_ERR_COUNT to value 0"]
impl crate::Resettable for RxErrCountSpec {}
