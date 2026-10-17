#[doc = "Register `UART_SW_MODE` reader"]
pub type R = crate::R<UartSwModeSpec>;
#[doc = "Register `UART_SW_MODE` writer"]
pub type W = crate::W<UartSwModeSpec>;
#[doc = "Field `cr_utx_txd_sw_mode` reader - "]
pub type CrUtxTxdSwModeR = crate::BitReader;
#[doc = "Field `cr_utx_txd_sw_mode` writer - "]
pub type CrUtxTxdSwModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_utx_txd_sw_val` reader - "]
pub type CrUtxTxdSwValR = crate::BitReader;
#[doc = "Field `cr_utx_txd_sw_val` writer - "]
pub type CrUtxTxdSwValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_rts_sw_mode` reader - "]
pub type CrUrxRtsSwModeR = crate::BitReader;
#[doc = "Field `cr_urx_rts_sw_mode` writer - "]
pub type CrUrxRtsSwModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_rts_sw_val` reader - "]
pub type CrUrxRtsSwValR = crate::BitReader;
#[doc = "Field `cr_urx_rts_sw_val` writer - "]
pub type CrUrxRtsSwValW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_txd_sw_mode(&self) -> CrUtxTxdSwModeR {
        CrUtxTxdSwModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_utx_txd_sw_val(&self) -> CrUtxTxdSwValR {
        CrUtxTxdSwValR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_urx_rts_sw_mode(&self) -> CrUrxRtsSwModeR {
        CrUrxRtsSwModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_urx_rts_sw_val(&self) -> CrUrxRtsSwValR {
        CrUrxRtsSwValR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_txd_sw_mode(&mut self) -> CrUtxTxdSwModeW<'_, UartSwModeSpec> {
        CrUtxTxdSwModeW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_utx_txd_sw_val(&mut self) -> CrUtxTxdSwValW<'_, UartSwModeSpec> {
        CrUtxTxdSwValW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_urx_rts_sw_mode(&mut self) -> CrUrxRtsSwModeW<'_, UartSwModeSpec> {
        CrUrxRtsSwModeW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_urx_rts_sw_val(&mut self) -> CrUrxRtsSwValW<'_, UartSwModeSpec> {
        CrUrxRtsSwValW::new(self, 3)
    }
}
#[doc = "UART Software Control Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_sw_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_sw_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartSwModeSpec;
impl crate::RegisterSpec for UartSwModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_sw_mode::R`](R) reader structure"]
impl crate::Readable for UartSwModeSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_sw_mode::W`](W) writer structure"]
impl crate::Writable for UartSwModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_SW_MODE to value 0"]
impl crate::Resettable for UartSwModeSpec {}
