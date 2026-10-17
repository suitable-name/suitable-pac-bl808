#[doc = "Register `TZC_BMX_S0` reader"]
pub type R = crate::R<TzcBmxS0Spec>;
#[doc = "Register `TZC_BMX_S0` writer"]
pub type W = crate::W<TzcBmxS0Spec>;
#[doc = "Field `tzc_bmx_mm_tzsid_en` reader - MM TZSID enable (bits \\[1:0\\] = R/W permission)"]
pub type TzcBmxMmTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_mm_tzsid_en` writer - MM TZSID enable (bits \\[1:0\\] = R/W permission)"]
pub type TzcBmxMmTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_bmx_dma_tzsid_en` reader - DMA TZSID enable"]
pub type TzcBmxDmaTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_dma_tzsid_en` writer - DMA TZSID enable"]
pub type TzcBmxDmaTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_bmx_dma2_tzsid_en` reader - DMA2 TZSID enable"]
pub type TzcBmxDma2TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_dma2_tzsid_en` writer - DMA2 TZSID enable"]
pub type TzcBmxDma2TzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_bmx_pwr_tzsid_en` reader - Power (PDS/HBN/AON) TZSID enable"]
pub type TzcBmxPwrTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_pwr_tzsid_en` writer - Power (PDS/HBN/AON) TZSID enable"]
pub type TzcBmxPwrTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_bmx_sdh_tzsid_en` reader - SD Host TZSID enable"]
pub type TzcBmxSdhTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_sdh_tzsid_en` writer - SD Host TZSID enable"]
pub type TzcBmxSdhTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_bmx_emac_tzsid_en` reader - EMAC TZSID enable"]
pub type TzcBmxEmacTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_emac_tzsid_en` writer - EMAC TZSID enable"]
pub type TzcBmxEmacTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_bmx_mm_tzsid_lock` reader - MM TZSID Lock (Write-Once)"]
pub type TzcBmxMmTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_mm_tzsid_lock` writer - MM TZSID Lock (Write-Once)"]
pub type TzcBmxMmTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_bmx_dma_tzsid_lock` reader - DMA TZSID Lock"]
pub type TzcBmxDmaTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_dma_tzsid_lock` writer - DMA TZSID Lock"]
pub type TzcBmxDmaTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_bmx_dma2_tzsid_lock` reader - DMA2 TZSID Lock"]
pub type TzcBmxDma2TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_dma2_tzsid_lock` writer - DMA2 TZSID Lock"]
pub type TzcBmxDma2TzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_bmx_pwr_tzsid_lock` reader - Power (PDS/HBN/AON) TZSID Lock"]
pub type TzcBmxPwrTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_pwr_tzsid_lock` writer - Power (PDS/HBN/AON) TZSID Lock"]
pub type TzcBmxPwrTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_bmx_sdh_tzsid_lock` reader - SDH TZSID Lock"]
pub type TzcBmxSdhTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_sdh_tzsid_lock` writer - SDH TZSID Lock"]
pub type TzcBmxSdhTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_bmx_emac_tzsid_lock` reader - EMAC TZSID Lock"]
pub type TzcBmxEmacTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_emac_tzsid_lock` writer - EMAC TZSID Lock"]
pub type TzcBmxEmacTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - MM TZSID enable (bits \\[1:0\\] = R/W permission)"]
    #[inline(always)]
    pub fn tzc_bmx_mm_tzsid_en(&self) -> TzcBmxMmTzsidEnR {
        TzcBmxMmTzsidEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - DMA TZSID enable"]
    #[inline(always)]
    pub fn tzc_bmx_dma_tzsid_en(&self) -> TzcBmxDmaTzsidEnR {
        TzcBmxDmaTzsidEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - DMA2 TZSID enable"]
    #[inline(always)]
    pub fn tzc_bmx_dma2_tzsid_en(&self) -> TzcBmxDma2TzsidEnR {
        TzcBmxDma2TzsidEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Power (PDS/HBN/AON) TZSID enable"]
    #[inline(always)]
    pub fn tzc_bmx_pwr_tzsid_en(&self) -> TzcBmxPwrTzsidEnR {
        TzcBmxPwrTzsidEnR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SD Host TZSID enable"]
    #[inline(always)]
    pub fn tzc_bmx_sdh_tzsid_en(&self) -> TzcBmxSdhTzsidEnR {
        TzcBmxSdhTzsidEnR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - EMAC TZSID enable"]
    #[inline(always)]
    pub fn tzc_bmx_emac_tzsid_en(&self) -> TzcBmxEmacTzsidEnR {
        TzcBmxEmacTzsidEnR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 16 - MM TZSID Lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_bmx_mm_tzsid_lock(&self) -> TzcBmxMmTzsidLockR {
        TzcBmxMmTzsidLockR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA TZSID Lock"]
    #[inline(always)]
    pub fn tzc_bmx_dma_tzsid_lock(&self) -> TzcBmxDmaTzsidLockR {
        TzcBmxDmaTzsidLockR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DMA2 TZSID Lock"]
    #[inline(always)]
    pub fn tzc_bmx_dma2_tzsid_lock(&self) -> TzcBmxDma2TzsidLockR {
        TzcBmxDma2TzsidLockR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Power (PDS/HBN/AON) TZSID Lock"]
    #[inline(always)]
    pub fn tzc_bmx_pwr_tzsid_lock(&self) -> TzcBmxPwrTzsidLockR {
        TzcBmxPwrTzsidLockR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SDH TZSID Lock"]
    #[inline(always)]
    pub fn tzc_bmx_sdh_tzsid_lock(&self) -> TzcBmxSdhTzsidLockR {
        TzcBmxSdhTzsidLockR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - EMAC TZSID Lock"]
    #[inline(always)]
    pub fn tzc_bmx_emac_tzsid_lock(&self) -> TzcBmxEmacTzsidLockR {
        TzcBmxEmacTzsidLockR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - MM TZSID enable (bits \\[1:0\\] = R/W permission)"]
    #[inline(always)]
    pub fn tzc_bmx_mm_tzsid_en(&mut self) -> TzcBmxMmTzsidEnW<'_, TzcBmxS0Spec> {
        TzcBmxMmTzsidEnW::new(self, 0)
    }
    #[doc = "Bits 2:3 - DMA TZSID enable"]
    #[inline(always)]
    pub fn tzc_bmx_dma_tzsid_en(&mut self) -> TzcBmxDmaTzsidEnW<'_, TzcBmxS0Spec> {
        TzcBmxDmaTzsidEnW::new(self, 2)
    }
    #[doc = "Bits 4:5 - DMA2 TZSID enable"]
    #[inline(always)]
    pub fn tzc_bmx_dma2_tzsid_en(&mut self) -> TzcBmxDma2TzsidEnW<'_, TzcBmxS0Spec> {
        TzcBmxDma2TzsidEnW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Power (PDS/HBN/AON) TZSID enable"]
    #[inline(always)]
    pub fn tzc_bmx_pwr_tzsid_en(&mut self) -> TzcBmxPwrTzsidEnW<'_, TzcBmxS0Spec> {
        TzcBmxPwrTzsidEnW::new(self, 6)
    }
    #[doc = "Bits 8:9 - SD Host TZSID enable"]
    #[inline(always)]
    pub fn tzc_bmx_sdh_tzsid_en(&mut self) -> TzcBmxSdhTzsidEnW<'_, TzcBmxS0Spec> {
        TzcBmxSdhTzsidEnW::new(self, 8)
    }
    #[doc = "Bits 10:11 - EMAC TZSID enable"]
    #[inline(always)]
    pub fn tzc_bmx_emac_tzsid_en(&mut self) -> TzcBmxEmacTzsidEnW<'_, TzcBmxS0Spec> {
        TzcBmxEmacTzsidEnW::new(self, 10)
    }
    #[doc = "Bit 16 - MM TZSID Lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_bmx_mm_tzsid_lock(&mut self) -> TzcBmxMmTzsidLockW<'_, TzcBmxS0Spec> {
        TzcBmxMmTzsidLockW::new(self, 16)
    }
    #[doc = "Bit 17 - DMA TZSID Lock"]
    #[inline(always)]
    pub fn tzc_bmx_dma_tzsid_lock(&mut self) -> TzcBmxDmaTzsidLockW<'_, TzcBmxS0Spec> {
        TzcBmxDmaTzsidLockW::new(self, 17)
    }
    #[doc = "Bit 18 - DMA2 TZSID Lock"]
    #[inline(always)]
    pub fn tzc_bmx_dma2_tzsid_lock(&mut self) -> TzcBmxDma2TzsidLockW<'_, TzcBmxS0Spec> {
        TzcBmxDma2TzsidLockW::new(self, 18)
    }
    #[doc = "Bit 19 - Power (PDS/HBN/AON) TZSID Lock"]
    #[inline(always)]
    pub fn tzc_bmx_pwr_tzsid_lock(&mut self) -> TzcBmxPwrTzsidLockW<'_, TzcBmxS0Spec> {
        TzcBmxPwrTzsidLockW::new(self, 19)
    }
    #[doc = "Bit 20 - SDH TZSID Lock"]
    #[inline(always)]
    pub fn tzc_bmx_sdh_tzsid_lock(&mut self) -> TzcBmxSdhTzsidLockW<'_, TzcBmxS0Spec> {
        TzcBmxSdhTzsidLockW::new(self, 20)
    }
    #[doc = "Bit 21 - EMAC TZSID Lock"]
    #[inline(always)]
    pub fn tzc_bmx_emac_tzsid_lock(&mut self) -> TzcBmxEmacTzsidLockW<'_, TzcBmxS0Spec> {
        TzcBmxEmacTzsidLockW::new(self, 21)
    }
}
#[doc = "Bus Matrix Slave 0 Access Control Enable (MM, DMA, PWR, SDH, EMAC)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_bmx_s0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_bmx_s0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcBmxS0Spec;
impl crate::RegisterSpec for TzcBmxS0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_bmx_s0::R`](R) reader structure"]
impl crate::Readable for TzcBmxS0Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_bmx_s0::W`](W) writer structure"]
impl crate::Writable for TzcBmxS0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_BMX_S0 to value 0x0fff"]
impl crate::Resettable for TzcBmxS0Spec {
    const RESET_VALUE: u32 = 0x0fff;
}
