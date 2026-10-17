#[doc = "Register `RX_MESSAGE_COUNT` reader"]
pub type R = crate::R<RxMessageCountSpec>;
#[doc = "Field `rx_message_cnt` reader - "]
pub type RxMessageCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rx_message_cnt(&self) -> RxMessageCntR {
        RxMessageCntR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Receive Message Counter Register. Indicates the number of messages currently stored in the receive FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_message_count::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxMessageCountSpec;
impl crate::RegisterSpec for RxMessageCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_message_count::R`](R) reader structure"]
impl crate::Readable for RxMessageCountSpec {}
#[doc = "`reset()` method sets RX_MESSAGE_COUNT to value 0"]
impl crate::Resettable for RxMessageCountSpec {}
