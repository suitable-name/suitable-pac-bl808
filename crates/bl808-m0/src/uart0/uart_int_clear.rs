#[doc = "Register `UART_INT_CLEAR` writer"]
pub type W = crate::W<UartIntClearSpec>;
#[doc = "Field `cr_utx_end_clr` writer - "]
pub type CrUtxEndClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_urx_end_clr` writer - "]
pub type CrUrxEndClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_urx_rto_clr` writer - "]
pub type CrUrxRtoClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_urx_pce_clr` writer - "]
pub type CrUrxPceClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_urx_lse_clr` writer - "]
pub type CrUrxLseClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_urx_bcr_clr` writer - "]
pub type CrUrxBcrClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_urx_ads_clr` writer - "]
pub type CrUrxAdsClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_urx_ad5_clr` writer - "]
pub type CrUrxAd5ClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_end_clr(&mut self) -> CrUtxEndClrW<'_, UartIntClearSpec> {
        CrUtxEndClrW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_end_clr(&mut self) -> CrUrxEndClrW<'_, UartIntClearSpec> {
        CrUrxEndClrW::new(self, 1)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_rto_clr(&mut self) -> CrUrxRtoClrW<'_, UartIntClearSpec> {
        CrUrxRtoClrW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_pce_clr(&mut self) -> CrUrxPceClrW<'_, UartIntClearSpec> {
        CrUrxPceClrW::new(self, 5)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_urx_lse_clr(&mut self) -> CrUrxLseClrW<'_, UartIntClearSpec> {
        CrUrxLseClrW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_urx_bcr_clr(&mut self) -> CrUrxBcrClrW<'_, UartIntClearSpec> {
        CrUrxBcrClrW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_urx_ads_clr(&mut self) -> CrUrxAdsClrW<'_, UartIntClearSpec> {
        CrUrxAdsClrW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_urx_ad5_clr(&mut self) -> CrUrxAd5ClrW<'_, UartIntClearSpec> {
        CrUrxAd5ClrW::new(self, 11)
    }
}
#[doc = "UART Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_int_clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartIntClearSpec;
impl crate::RegisterSpec for UartIntClearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uart_int_clear::W`](W) writer structure"]
impl crate::Writable for UartIntClearSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f33;
}
#[doc = "`reset()` method sets UART_INT_CLEAR to value 0"]
impl crate::Resettable for UartIntClearSpec {}
