#[doc = "Register `DBI_FIFO_CONFIG_1` reader"]
pub type R = crate::R<DbiFifoConfig1Spec>;
#[doc = "Register `DBI_FIFO_CONFIG_1` writer"]
pub type W = crate::W<DbiFifoConfig1Spec>;
#[doc = "Field `tx_fifo_cnt` reader - TX FIFO count"]
pub type TxFifoCntR = crate::FieldReader;
#[doc = "Field `tx_fifo_th` reader - TX FIFO interrupt threshold"]
pub type TxFifoThR = crate::FieldReader;
#[doc = "Field `tx_fifo_th` writer - TX FIFO interrupt threshold"]
pub type TxFifoThW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - TX FIFO count"]
    #[inline(always)]
    pub fn tx_fifo_cnt(&self) -> TxFifoCntR {
        TxFifoCntR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - TX FIFO interrupt threshold"]
    #[inline(always)]
    pub fn tx_fifo_th(&self) -> TxFifoThR {
        TxFifoThR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - TX FIFO interrupt threshold"]
    #[inline(always)]
    pub fn tx_fifo_th(&mut self) -> TxFifoThW<'_, DbiFifoConfig1Spec> {
        TxFifoThW::new(self, 16)
    }
}
#[doc = "DBI FIFO Status and Thresholds\n\nYou can [`read`](crate::Reg::read) this register and get [`dbi_fifo_config_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbi_fifo_config_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbiFifoConfig1Spec;
impl crate::RegisterSpec for DbiFifoConfig1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbi_fifo_config_1::R`](R) reader structure"]
impl crate::Readable for DbiFifoConfig1Spec {}
#[doc = "`write(|w| ..)` method takes [`dbi_fifo_config_1::W`](W) writer structure"]
impl crate::Writable for DbiFifoConfig1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBI_FIFO_CONFIG_1 to value 0x08"]
impl crate::Resettable for DbiFifoConfig1Spec {
    const RESET_VALUE: u32 = 0x08;
}
