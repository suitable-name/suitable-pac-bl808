#[doc = "Register `SNSR2DVP_WAIT_POS` reader"]
pub type R = crate::R<Snsr2dvpWaitPosSpec>;
#[doc = "Register `SNSR2DVP_WAIT_POS` writer"]
pub type W = crate::W<Snsr2dvpWaitPosSpec>;
#[doc = "Field `cr_snsr_fifo_th` reader - Sensor FIFO threshold for flow control"]
pub type CrSnsrFifoThR = crate::FieldReader<u16>;
#[doc = "Field `cr_snsr_fifo_th` writer - Sensor FIFO threshold for flow control"]
pub type CrSnsrFifoThW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Sensor FIFO threshold for flow control"]
    #[inline(always)]
    pub fn cr_snsr_fifo_th(&self) -> CrSnsrFifoThR {
        CrSnsrFifoThR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Sensor FIFO threshold for flow control"]
    #[inline(always)]
    pub fn cr_snsr_fifo_th(&mut self) -> CrSnsrFifoThW<'_, Snsr2dvpWaitPosSpec> {
        CrSnsrFifoThW::new(self, 0)
    }
}
#[doc = "Sensor to DVP FIFO Threshold/Wait Position\n\nYou can [`read`](crate::Reg::read) this register and get [`snsr2dvp_wait_pos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snsr2dvp_wait_pos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Snsr2dvpWaitPosSpec;
impl crate::RegisterSpec for Snsr2dvpWaitPosSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`snsr2dvp_wait_pos::R`](R) reader structure"]
impl crate::Readable for Snsr2dvpWaitPosSpec {}
#[doc = "`write(|w| ..)` method takes [`snsr2dvp_wait_pos::W`](W) writer structure"]
impl crate::Writable for Snsr2dvpWaitPosSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SNSR2DVP_WAIT_POS to value 0x8b"]
impl crate::Resettable for Snsr2dvpWaitPosSpec {
    const RESET_VALUE: u32 = 0x8b;
}
