#[doc = "Register `TZC_OCRAM_TZSRG_CTRL` reader"]
pub type R = crate::R<TzcOcramTzsrgCtrlSpec>;
#[doc = "Field `tzc_ocram_tzsrg_r0_id_en` reader - "]
pub type TzcOcramTzsrgR0IdEnR = crate::FieldReader;
#[doc = "Field `tzc_ocram_tzsrg_r1_id_en` reader - "]
pub type TzcOcramTzsrgR1IdEnR = crate::FieldReader;
#[doc = "Field `tzc_ocram_tzsrg_r2_id_en` reader - "]
pub type TzcOcramTzsrgR2IdEnR = crate::FieldReader;
#[doc = "Field `tzc_ocram_tzsrg_rx_id_en` reader - RX ID Enable"]
pub type TzcOcramTzsrgRxIdEnR = crate::FieldReader;
#[doc = "Field `tzc_ocram_tzsrg_r0_en` reader - "]
pub type TzcOcramTzsrgR0EnR = crate::BitReader;
#[doc = "Field `tzc_ocram_tzsrg_r1_en` reader - "]
pub type TzcOcramTzsrgR1EnR = crate::BitReader;
#[doc = "Field `tzc_ocram_tzsrg_r2_en` reader - "]
pub type TzcOcramTzsrgR2EnR = crate::BitReader;
#[doc = "Field `tzc_ocram_tzsrg_rx_en` reader - RX Enable"]
pub type TzcOcramTzsrgRxEnR = crate::BitReader;
#[doc = "Field `tzc_ocram_tzsrg_r0_lock` reader - "]
pub type TzcOcramTzsrgR0LockR = crate::BitReader;
#[doc = "Field `tzc_ocram_tzsrg_r1_lock` reader - "]
pub type TzcOcramTzsrgR1LockR = crate::BitReader;
#[doc = "Field `tzc_ocram_tzsrg_r2_lock` reader - "]
pub type TzcOcramTzsrgR2LockR = crate::BitReader;
#[doc = "Field `tzc_ocram_tzsrg_rx_lock` reader - RX Lock"]
pub type TzcOcramTzsrgRxLockR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r0_id_en(&self) -> TzcOcramTzsrgR0IdEnR {
        TzcOcramTzsrgR0IdEnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r1_id_en(&self) -> TzcOcramTzsrgR1IdEnR {
        TzcOcramTzsrgR1IdEnR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r2_id_en(&self) -> TzcOcramTzsrgR2IdEnR {
        TzcOcramTzsrgR2IdEnR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - RX ID Enable"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_rx_id_en(&self) -> TzcOcramTzsrgRxIdEnR {
        TzcOcramTzsrgRxIdEnR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r0_en(&self) -> TzcOcramTzsrgR0EnR {
        TzcOcramTzsrgR0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r1_en(&self) -> TzcOcramTzsrgR1EnR {
        TzcOcramTzsrgR1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r2_en(&self) -> TzcOcramTzsrgR2EnR {
        TzcOcramTzsrgR2EnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RX Enable"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_rx_en(&self) -> TzcOcramTzsrgRxEnR {
        TzcOcramTzsrgRxEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r0_lock(&self) -> TzcOcramTzsrgR0LockR {
        TzcOcramTzsrgR0LockR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r1_lock(&self) -> TzcOcramTzsrgR1LockR {
        TzcOcramTzsrgR1LockR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r2_lock(&self) -> TzcOcramTzsrgR2LockR {
        TzcOcramTzsrgR2LockR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - RX Lock"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_rx_lock(&self) -> TzcOcramTzsrgRxLockR {
        TzcOcramTzsrgRxLockR::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "OCRAM TZSRG Control Register (Region Enable/ID Enable/Lock)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_ocram_tzsrg_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcOcramTzsrgCtrlSpec;
impl crate::RegisterSpec for TzcOcramTzsrgCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_ocram_tzsrg_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcOcramTzsrgCtrlSpec {}
#[doc = "`reset()` method sets TZC_OCRAM_TZSRG_CTRL to value 0"]
impl crate::Resettable for TzcOcramTzsrgCtrlSpec {}
