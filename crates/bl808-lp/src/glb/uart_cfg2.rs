#[doc = "Register `UART_CFG2` reader"]
pub type R = crate::R<UartCfg2Spec>;
#[doc = "Register `UART_CFG2` writer"]
pub type W = crate::W<UartCfg2Spec>;
#[doc = "Field `uart_sig_8_sel` reader - "]
pub type UartSig8SelR = crate::FieldReader;
#[doc = "Field `uart_sig_8_sel` writer - "]
pub type UartSig8SelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `uart_sig_9_sel` reader - "]
pub type UartSig9SelR = crate::FieldReader;
#[doc = "Field `uart_sig_9_sel` writer - "]
pub type UartSig9SelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `uart_sig_10_sel` reader - "]
pub type UartSig10SelR = crate::FieldReader;
#[doc = "Field `uart_sig_10_sel` writer - "]
pub type UartSig10SelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `uart_sig_11_sel` reader - "]
pub type UartSig11SelR = crate::FieldReader;
#[doc = "Field `uart_sig_11_sel` writer - "]
pub type UartSig11SelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn uart_sig_8_sel(&self) -> UartSig8SelR {
        UartSig8SelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn uart_sig_9_sel(&self) -> UartSig9SelR {
        UartSig9SelR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn uart_sig_10_sel(&self) -> UartSig10SelR {
        UartSig10SelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn uart_sig_11_sel(&self) -> UartSig11SelR {
        UartSig11SelR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn uart_sig_8_sel(&mut self) -> UartSig8SelW<'_, UartCfg2Spec> {
        UartSig8SelW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn uart_sig_9_sel(&mut self) -> UartSig9SelW<'_, UartCfg2Spec> {
        UartSig9SelW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn uart_sig_10_sel(&mut self) -> UartSig10SelW<'_, UartCfg2Spec> {
        UartSig10SelW::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn uart_sig_11_sel(&mut self) -> UartSig11SelW<'_, UartCfg2Spec> {
        UartSig11SelW::new(self, 12)
    }
}
#[doc = "UART Signal Mux 8-11\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartCfg2Spec;
impl crate::RegisterSpec for UartCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_cfg2::R`](R) reader structure"]
impl crate::Readable for UartCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`uart_cfg2::W`](W) writer structure"]
impl crate::Writable for UartCfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_CFG2 to value 0"]
impl crate::Resettable for UartCfg2Spec {}
