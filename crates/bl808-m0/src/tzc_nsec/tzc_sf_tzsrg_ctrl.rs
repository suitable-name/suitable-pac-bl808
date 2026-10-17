#[doc = "Register `TZC_SF_TZSRG_CTRL` reader"]
pub type R = crate::R<TzcSfTzsrgCtrlSpec>;
#[doc = "Field `tzc_sf_tzsrg_r0_id_en` reader - "]
pub type TzcSfTzsrgR0IdEnR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r1_id_en` reader - "]
pub type TzcSfTzsrgR1IdEnR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r2_id_en` reader - "]
pub type TzcSfTzsrgR2IdEnR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r3_id_en` reader - "]
pub type TzcSfTzsrgR3IdEnR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_rx_id_en` reader - "]
pub type TzcSfTzsrgRxIdEnR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r0_en` reader - "]
pub type TzcSfTzsrgR0EnR = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_r1_en` reader - "]
pub type TzcSfTzsrgR1EnR = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_r2_en` reader - "]
pub type TzcSfTzsrgR2EnR = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_r3_en` reader - "]
pub type TzcSfTzsrgR3EnR = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_rx_en` reader - "]
pub type TzcSfTzsrgRxEnR = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_r0_lock` reader - "]
pub type TzcSfTzsrgR0LockR = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_r1_lock` reader - "]
pub type TzcSfTzsrgR1LockR = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_r2_lock` reader - "]
pub type TzcSfTzsrgR2LockR = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_r3_lock` reader - "]
pub type TzcSfTzsrgR3LockR = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_rx_lock` reader - "]
pub type TzcSfTzsrgRxLockR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r0_id_en(&self) -> TzcSfTzsrgR0IdEnR {
        TzcSfTzsrgR0IdEnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_id_en(&self) -> TzcSfTzsrgR1IdEnR {
        TzcSfTzsrgR1IdEnR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_id_en(&self) -> TzcSfTzsrgR2IdEnR {
        TzcSfTzsrgR2IdEnR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_id_en(&self) -> TzcSfTzsrgR3IdEnR {
        TzcSfTzsrgR3IdEnR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_rx_id_en(&self) -> TzcSfTzsrgRxIdEnR {
        TzcSfTzsrgRxIdEnR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r0_en(&self) -> TzcSfTzsrgR0EnR {
        TzcSfTzsrgR0EnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_en(&self) -> TzcSfTzsrgR1EnR {
        TzcSfTzsrgR1EnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_en(&self) -> TzcSfTzsrgR2EnR {
        TzcSfTzsrgR2EnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_en(&self) -> TzcSfTzsrgR3EnR {
        TzcSfTzsrgR3EnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_rx_en(&self) -> TzcSfTzsrgRxEnR {
        TzcSfTzsrgRxEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r0_lock(&self) -> TzcSfTzsrgR0LockR {
        TzcSfTzsrgR0LockR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_lock(&self) -> TzcSfTzsrgR1LockR {
        TzcSfTzsrgR1LockR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_lock(&self) -> TzcSfTzsrgR2LockR {
        TzcSfTzsrgR2LockR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_lock(&self) -> TzcSfTzsrgR3LockR {
        TzcSfTzsrgR3LockR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_rx_lock(&self) -> TzcSfTzsrgRxLockR {
        TzcSfTzsrgRxLockR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "SF TZSRG Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcSfTzsrgCtrlSpec;
impl crate::RegisterSpec for TzcSfTzsrgCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_sf_tzsrg_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcSfTzsrgCtrlSpec {}
#[doc = "`reset()` method sets TZC_SF_TZSRG_CTRL to value 0"]
impl crate::Resettable for TzcSfTzsrgCtrlSpec {}
