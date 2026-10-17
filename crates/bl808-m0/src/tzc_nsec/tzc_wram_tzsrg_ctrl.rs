#[doc = "Register `TZC_WRAM_TZSRG_CTRL` reader"]
pub type R = crate::R<TzcWramTzsrgCtrlSpec>;
#[doc = "Field `tzc_wram_tzsrg_r0_id_en` reader - "]
pub type TzcWramTzsrgR0IdEnR = crate::FieldReader;
#[doc = "Field `tzc_wram_tzsrg_r1_id_en` reader - "]
pub type TzcWramTzsrgR1IdEnR = crate::FieldReader;
#[doc = "Field `tzc_wram_tzsrg_r2_id_en` reader - "]
pub type TzcWramTzsrgR2IdEnR = crate::FieldReader;
#[doc = "Field `tzc_wram_tzsrg_rx_id_en` reader - RX ID Enable"]
pub type TzcWramTzsrgRxIdEnR = crate::FieldReader;
#[doc = "Field `tzc_wram_tzsrg_r0_en` reader - "]
pub type TzcWramTzsrgR0EnR = crate::BitReader;
#[doc = "Field `tzc_wram_tzsrg_r1_en` reader - "]
pub type TzcWramTzsrgR1EnR = crate::BitReader;
#[doc = "Field `tzc_wram_tzsrg_r2_en` reader - "]
pub type TzcWramTzsrgR2EnR = crate::BitReader;
#[doc = "Field `tzc_wram_tzsrg_rx_en` reader - RX Enable"]
pub type TzcWramTzsrgRxEnR = crate::BitReader;
#[doc = "Field `tzc_wram_tzsrg_r0_lock` reader - "]
pub type TzcWramTzsrgR0LockR = crate::BitReader;
#[doc = "Field `tzc_wram_tzsrg_r1_lock` reader - "]
pub type TzcWramTzsrgR1LockR = crate::BitReader;
#[doc = "Field `tzc_wram_tzsrg_r2_lock` reader - "]
pub type TzcWramTzsrgR2LockR = crate::BitReader;
#[doc = "Field `tzc_wram_tzsrg_rx_lock` reader - RX Lock"]
pub type TzcWramTzsrgRxLockR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r0_id_en(&self) -> TzcWramTzsrgR0IdEnR {
        TzcWramTzsrgR0IdEnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r1_id_en(&self) -> TzcWramTzsrgR1IdEnR {
        TzcWramTzsrgR1IdEnR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r2_id_en(&self) -> TzcWramTzsrgR2IdEnR {
        TzcWramTzsrgR2IdEnR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - RX ID Enable"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_rx_id_en(&self) -> TzcWramTzsrgRxIdEnR {
        TzcWramTzsrgRxIdEnR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r0_en(&self) -> TzcWramTzsrgR0EnR {
        TzcWramTzsrgR0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r1_en(&self) -> TzcWramTzsrgR1EnR {
        TzcWramTzsrgR1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r2_en(&self) -> TzcWramTzsrgR2EnR {
        TzcWramTzsrgR2EnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RX Enable"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_rx_en(&self) -> TzcWramTzsrgRxEnR {
        TzcWramTzsrgRxEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r0_lock(&self) -> TzcWramTzsrgR0LockR {
        TzcWramTzsrgR0LockR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r1_lock(&self) -> TzcWramTzsrgR1LockR {
        TzcWramTzsrgR1LockR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r2_lock(&self) -> TzcWramTzsrgR2LockR {
        TzcWramTzsrgR2LockR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - RX Lock"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_rx_lock(&self) -> TzcWramTzsrgRxLockR {
        TzcWramTzsrgRxLockR::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "WRAM (MCU/Wireless RAM) TZSRG Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_wram_tzsrg_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcWramTzsrgCtrlSpec;
impl crate::RegisterSpec for TzcWramTzsrgCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_wram_tzsrg_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcWramTzsrgCtrlSpec {}
#[doc = "`reset()` method sets TZC_WRAM_TZSRG_CTRL to value 0"]
impl crate::Resettable for TzcWramTzsrgCtrlSpec {}
