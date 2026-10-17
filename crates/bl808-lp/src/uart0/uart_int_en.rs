#[doc = "Register `UART_INT_EN` reader"]
pub type R = crate::R<UartIntEnSpec>;
#[doc = "Register `UART_INT_EN` writer"]
pub type W = crate::W<UartIntEnSpec>;
#[doc = "Field `cr_utx_end_en` reader - "]
pub type CrUtxEndEnR = crate::BitReader;
#[doc = "Field `cr_utx_end_en` writer - "]
pub type CrUtxEndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_end_en` reader - "]
pub type CrUrxEndEnR = crate::BitReader;
#[doc = "Field `cr_urx_end_en` writer - "]
pub type CrUrxEndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_utx_frdy_en` reader - "]
pub type CrUtxFrdyEnR = crate::BitReader;
#[doc = "Field `cr_utx_frdy_en` writer - "]
pub type CrUtxFrdyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_frdy_en` reader - "]
pub type CrUrxFrdyEnR = crate::BitReader;
#[doc = "Field `cr_urx_frdy_en` writer - "]
pub type CrUrxFrdyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_rto_en` reader - "]
pub type CrUrxRtoEnR = crate::BitReader;
#[doc = "Field `cr_urx_rto_en` writer - "]
pub type CrUrxRtoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_pce_en` reader - "]
pub type CrUrxPceEnR = crate::BitReader;
#[doc = "Field `cr_urx_pce_en` writer - "]
pub type CrUrxPceEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_utx_fer_en` reader - "]
pub type CrUtxFerEnR = crate::BitReader;
#[doc = "Field `cr_utx_fer_en` writer - "]
pub type CrUtxFerEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_fer_en` reader - "]
pub type CrUrxFerEnR = crate::BitReader;
#[doc = "Field `cr_urx_fer_en` writer - "]
pub type CrUrxFerEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_lse_en` reader - "]
pub type CrUrxLseEnR = crate::BitReader;
#[doc = "Field `cr_urx_lse_en` writer - "]
pub type CrUrxLseEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_bcr_en` reader - "]
pub type CrUrxBcrEnR = crate::BitReader;
#[doc = "Field `cr_urx_bcr_en` writer - "]
pub type CrUrxBcrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_ads_en` reader - "]
pub type CrUrxAdsEnR = crate::BitReader;
#[doc = "Field `cr_urx_ads_en` writer - "]
pub type CrUrxAdsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_ad5_en` reader - "]
pub type CrUrxAd5EnR = crate::BitReader;
#[doc = "Field `cr_urx_ad5_en` writer - "]
pub type CrUrxAd5EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_end_en(&self) -> CrUtxEndEnR {
        CrUtxEndEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_end_en(&self) -> CrUrxEndEnR {
        CrUrxEndEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_utx_frdy_en(&self) -> CrUtxFrdyEnR {
        CrUtxFrdyEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_urx_frdy_en(&self) -> CrUrxFrdyEnR {
        CrUrxFrdyEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_rto_en(&self) -> CrUrxRtoEnR {
        CrUrxRtoEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_pce_en(&self) -> CrUrxPceEnR {
        CrUrxPceEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_utx_fer_en(&self) -> CrUtxFerEnR {
        CrUtxFerEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_urx_fer_en(&self) -> CrUrxFerEnR {
        CrUrxFerEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_urx_lse_en(&self) -> CrUrxLseEnR {
        CrUrxLseEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_urx_bcr_en(&self) -> CrUrxBcrEnR {
        CrUrxBcrEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_urx_ads_en(&self) -> CrUrxAdsEnR {
        CrUrxAdsEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_urx_ad5_en(&self) -> CrUrxAd5EnR {
        CrUrxAd5EnR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_end_en(&mut self) -> CrUtxEndEnW<'_, UartIntEnSpec> {
        CrUtxEndEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_end_en(&mut self) -> CrUrxEndEnW<'_, UartIntEnSpec> {
        CrUrxEndEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_utx_frdy_en(&mut self) -> CrUtxFrdyEnW<'_, UartIntEnSpec> {
        CrUtxFrdyEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_urx_frdy_en(&mut self) -> CrUrxFrdyEnW<'_, UartIntEnSpec> {
        CrUrxFrdyEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_rto_en(&mut self) -> CrUrxRtoEnW<'_, UartIntEnSpec> {
        CrUrxRtoEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_pce_en(&mut self) -> CrUrxPceEnW<'_, UartIntEnSpec> {
        CrUrxPceEnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_utx_fer_en(&mut self) -> CrUtxFerEnW<'_, UartIntEnSpec> {
        CrUtxFerEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_urx_fer_en(&mut self) -> CrUrxFerEnW<'_, UartIntEnSpec> {
        CrUrxFerEnW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_urx_lse_en(&mut self) -> CrUrxLseEnW<'_, UartIntEnSpec> {
        CrUrxLseEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_urx_bcr_en(&mut self) -> CrUrxBcrEnW<'_, UartIntEnSpec> {
        CrUrxBcrEnW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_urx_ads_en(&mut self) -> CrUrxAdsEnW<'_, UartIntEnSpec> {
        CrUrxAdsEnW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_urx_ad5_en(&mut self) -> CrUrxAd5EnW<'_, UartIntEnSpec> {
        CrUrxAd5EnW::new(self, 11)
    }
}
#[doc = "UART Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartIntEnSpec;
impl crate::RegisterSpec for UartIntEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_int_en::R`](R) reader structure"]
impl crate::Readable for UartIntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_int_en::W`](W) writer structure"]
impl crate::Writable for UartIntEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_INT_EN to value 0x0fff"]
impl crate::Resettable for UartIntEnSpec {
    const RESET_VALUE: u32 = 0x0fff;
}
