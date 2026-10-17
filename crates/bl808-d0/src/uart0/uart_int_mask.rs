#[doc = "Register `UART_INT_MASK` reader"]
pub type R = crate::R<UartIntMaskSpec>;
#[doc = "Register `UART_INT_MASK` writer"]
pub type W = crate::W<UartIntMaskSpec>;
#[doc = "Field `cr_utx_end_mask` reader - "]
pub type CrUtxEndMaskR = crate::BitReader;
#[doc = "Field `cr_utx_end_mask` writer - "]
pub type CrUtxEndMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_end_mask` reader - "]
pub type CrUrxEndMaskR = crate::BitReader;
#[doc = "Field `cr_urx_end_mask` writer - "]
pub type CrUrxEndMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_utx_frdy_mask` reader - "]
pub type CrUtxFrdyMaskR = crate::BitReader;
#[doc = "Field `cr_utx_frdy_mask` writer - "]
pub type CrUtxFrdyMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_frdy_mask` reader - "]
pub type CrUrxFrdyMaskR = crate::BitReader;
#[doc = "Field `cr_urx_frdy_mask` writer - "]
pub type CrUrxFrdyMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_rto_mask` reader - "]
pub type CrUrxRtoMaskR = crate::BitReader;
#[doc = "Field `cr_urx_rto_mask` writer - "]
pub type CrUrxRtoMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_pce_mask` reader - "]
pub type CrUrxPceMaskR = crate::BitReader;
#[doc = "Field `cr_urx_pce_mask` writer - "]
pub type CrUrxPceMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_utx_fer_mask` reader - "]
pub type CrUtxFerMaskR = crate::BitReader;
#[doc = "Field `cr_utx_fer_mask` writer - "]
pub type CrUtxFerMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_fer_mask` reader - "]
pub type CrUrxFerMaskR = crate::BitReader;
#[doc = "Field `cr_urx_fer_mask` writer - "]
pub type CrUrxFerMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_lse_mask` reader - "]
pub type CrUrxLseMaskR = crate::BitReader;
#[doc = "Field `cr_urx_lse_mask` writer - "]
pub type CrUrxLseMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_bcr_mask` reader - "]
pub type CrUrxBcrMaskR = crate::BitReader;
#[doc = "Field `cr_urx_bcr_mask` writer - "]
pub type CrUrxBcrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_ads_mask` reader - "]
pub type CrUrxAdsMaskR = crate::BitReader;
#[doc = "Field `cr_urx_ads_mask` writer - "]
pub type CrUrxAdsMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_urx_ad5_mask` reader - "]
pub type CrUrxAd5MaskR = crate::BitReader;
#[doc = "Field `cr_urx_ad5_mask` writer - "]
pub type CrUrxAd5MaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_end_mask(&self) -> CrUtxEndMaskR {
        CrUtxEndMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_end_mask(&self) -> CrUrxEndMaskR {
        CrUrxEndMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_utx_frdy_mask(&self) -> CrUtxFrdyMaskR {
        CrUtxFrdyMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_urx_frdy_mask(&self) -> CrUrxFrdyMaskR {
        CrUrxFrdyMaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_rto_mask(&self) -> CrUrxRtoMaskR {
        CrUrxRtoMaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_pce_mask(&self) -> CrUrxPceMaskR {
        CrUrxPceMaskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_utx_fer_mask(&self) -> CrUtxFerMaskR {
        CrUtxFerMaskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_urx_fer_mask(&self) -> CrUrxFerMaskR {
        CrUrxFerMaskR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_urx_lse_mask(&self) -> CrUrxLseMaskR {
        CrUrxLseMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_urx_bcr_mask(&self) -> CrUrxBcrMaskR {
        CrUrxBcrMaskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_urx_ads_mask(&self) -> CrUrxAdsMaskR {
        CrUrxAdsMaskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_urx_ad5_mask(&self) -> CrUrxAd5MaskR {
        CrUrxAd5MaskR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_end_mask(&mut self) -> CrUtxEndMaskW<'_, UartIntMaskSpec> {
        CrUtxEndMaskW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_end_mask(&mut self) -> CrUrxEndMaskW<'_, UartIntMaskSpec> {
        CrUrxEndMaskW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_utx_frdy_mask(&mut self) -> CrUtxFrdyMaskW<'_, UartIntMaskSpec> {
        CrUtxFrdyMaskW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_urx_frdy_mask(&mut self) -> CrUrxFrdyMaskW<'_, UartIntMaskSpec> {
        CrUrxFrdyMaskW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_rto_mask(&mut self) -> CrUrxRtoMaskW<'_, UartIntMaskSpec> {
        CrUrxRtoMaskW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_pce_mask(&mut self) -> CrUrxPceMaskW<'_, UartIntMaskSpec> {
        CrUrxPceMaskW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_utx_fer_mask(&mut self) -> CrUtxFerMaskW<'_, UartIntMaskSpec> {
        CrUtxFerMaskW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_urx_fer_mask(&mut self) -> CrUrxFerMaskW<'_, UartIntMaskSpec> {
        CrUrxFerMaskW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_urx_lse_mask(&mut self) -> CrUrxLseMaskW<'_, UartIntMaskSpec> {
        CrUrxLseMaskW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_urx_bcr_mask(&mut self) -> CrUrxBcrMaskW<'_, UartIntMaskSpec> {
        CrUrxBcrMaskW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_urx_ads_mask(&mut self) -> CrUrxAdsMaskW<'_, UartIntMaskSpec> {
        CrUrxAdsMaskW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_urx_ad5_mask(&mut self) -> CrUrxAd5MaskW<'_, UartIntMaskSpec> {
        CrUrxAd5MaskW::new(self, 11)
    }
}
#[doc = "UART Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_int_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_int_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartIntMaskSpec;
impl crate::RegisterSpec for UartIntMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_int_mask::R`](R) reader structure"]
impl crate::Readable for UartIntMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_int_mask::W`](W) writer structure"]
impl crate::Writable for UartIntMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_INT_MASK to value 0x0fff"]
impl crate::Resettable for UartIntMaskSpec {
    const RESET_VALUE: u32 = 0x0fff;
}
