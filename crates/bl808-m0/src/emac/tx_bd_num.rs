#[doc = "Register `TX_BD_NUM` reader"]
pub type R = crate::R<TxBdNumSpec>;
#[doc = "Register `TX_BD_NUM` writer"]
pub type W = crate::W<TxBdNumSpec>;
#[doc = "Field `TXBDNUM` reader - Number of descriptors used for TX"]
pub type TxbdnumR = crate::FieldReader;
#[doc = "Field `TXBDNUM` writer - Number of descriptors used for TX"]
pub type TxbdnumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TXBDPTR` reader - TX BD current pointer status"]
pub type TxbdptrR = crate::FieldReader;
#[doc = "Field `RXBDPTR` reader - RX BD current pointer status"]
pub type RxbdptrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of descriptors used for TX"]
    #[inline(always)]
    pub fn txbdnum(&self) -> TxbdnumR {
        TxbdnumR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - TX BD current pointer status"]
    #[inline(always)]
    pub fn txbdptr(&self) -> TxbdptrR {
        TxbdptrR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - RX BD current pointer status"]
    #[inline(always)]
    pub fn rxbdptr(&self) -> RxbdptrR {
        RxbdptrR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of descriptors used for TX"]
    #[inline(always)]
    pub fn txbdnum(&mut self) -> TxbdnumW<'_, TxBdNumSpec> {
        TxbdnumW::new(self, 0)
    }
}
#[doc = "Transmit Buffer Descriptor Number and Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_bd_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_bd_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxBdNumSpec;
impl crate::RegisterSpec for TxBdNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_bd_num::R`](R) reader structure"]
impl crate::Readable for TxBdNumSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_bd_num::W`](W) writer structure"]
impl crate::Writable for TxBdNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_BD_NUM to value 0x40"]
impl crate::Resettable for TxBdNumSpec {
    const RESET_VALUE: u32 = 0x40;
}
