#[doc = "Register `UART_BIT_PRD` reader"]
pub type R = crate::R<UartBitPrdSpec>;
#[doc = "Register `UART_BIT_PRD` writer"]
pub type W = crate::W<UartBitPrdSpec>;
#[doc = "Field `cr_utx_bit_prd` reader - TX bit period in UART clock cycles"]
pub type CrUtxBitPrdR = crate::FieldReader<u16>;
#[doc = "Field `cr_utx_bit_prd` writer - TX bit period in UART clock cycles"]
pub type CrUtxBitPrdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `cr_urx_bit_prd` reader - RX bit period in UART clock cycles"]
pub type CrUrxBitPrdR = crate::FieldReader<u16>;
#[doc = "Field `cr_urx_bit_prd` writer - RX bit period in UART clock cycles"]
pub type CrUrxBitPrdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - TX bit period in UART clock cycles"]
    #[inline(always)]
    pub fn cr_utx_bit_prd(&self) -> CrUtxBitPrdR {
        CrUtxBitPrdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - RX bit period in UART clock cycles"]
    #[inline(always)]
    pub fn cr_urx_bit_prd(&self) -> CrUrxBitPrdR {
        CrUrxBitPrdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TX bit period in UART clock cycles"]
    #[inline(always)]
    pub fn cr_utx_bit_prd(&mut self) -> CrUtxBitPrdW<'_, UartBitPrdSpec> {
        CrUtxBitPrdW::new(self, 0)
    }
    #[doc = "Bits 16:31 - RX bit period in UART clock cycles"]
    #[inline(always)]
    pub fn cr_urx_bit_prd(&mut self) -> CrUrxBitPrdW<'_, UartBitPrdSpec> {
        CrUrxBitPrdW::new(self, 16)
    }
}
#[doc = "UART Baud Rate Bit Period\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_bit_prd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_bit_prd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartBitPrdSpec;
impl crate::RegisterSpec for UartBitPrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_bit_prd::R`](R) reader structure"]
impl crate::Readable for UartBitPrdSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_bit_prd::W`](W) writer structure"]
impl crate::Writable for UartBitPrdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_BIT_PRD to value 0x00ff_00ff"]
impl crate::Resettable for UartBitPrdSpec {
    const RESET_VALUE: u32 = 0x00ff_00ff;
}
