#[doc = "Register `DATA_CONFIG` reader"]
pub type R = crate::R<DataConfigSpec>;
#[doc = "Register `DATA_CONFIG` writer"]
pub type W = crate::W<DataConfigSpec>;
#[doc = "Field `cr_uart_bit_inv` reader - Invert data bit order: 0:LSB first, 1:MSB first"]
pub type CrUartBitInvR = crate::BitReader;
#[doc = "Field `cr_uart_bit_inv` writer - Invert data bit order: 0:LSB first, 1:MSB first"]
pub type CrUartBitInvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Invert data bit order: 0:LSB first, 1:MSB first"]
    #[inline(always)]
    pub fn cr_uart_bit_inv(&self) -> CrUartBitInvR {
        CrUartBitInvR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invert data bit order: 0:LSB first, 1:MSB first"]
    #[inline(always)]
    pub fn cr_uart_bit_inv(&mut self) -> CrUartBitInvW<'_, DataConfigSpec> {
        CrUartBitInvW::new(self, 0)
    }
}
#[doc = "UART Data Format Config\n\nYou can [`read`](crate::Reg::read) this register and get [`data_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataConfigSpec;
impl crate::RegisterSpec for DataConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_config::R`](R) reader structure"]
impl crate::Readable for DataConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`data_config::W`](W) writer structure"]
impl crate::Writable for DataConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_CONFIG to value 0"]
impl crate::Resettable for DataConfigSpec {}
