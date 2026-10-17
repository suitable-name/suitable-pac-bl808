#[doc = "Register `TZC_PSRAMB_TZSRG_CTRL` reader"]
pub type R = crate::R<TzcPsrambTzsrgCtrlSpec>;
#[doc = "Field `tzc_psramb_tzsrg_r0_id_en` reader - "]
pub type TzcPsrambTzsrgR0IdEnR = crate::FieldReader;
#[doc = "Field `tzc_psramb_tzsrg_r1_id_en` reader - "]
pub type TzcPsrambTzsrgR1IdEnR = crate::FieldReader;
#[doc = "Field `tzc_psramb_tzsrg_r2_id_en` reader - "]
pub type TzcPsrambTzsrgR2IdEnR = crate::FieldReader;
#[doc = "Field `tzc_psramb_tzsrg_r0_en` reader - "]
pub type TzcPsrambTzsrgR0EnR = crate::BitReader;
#[doc = "Field `tzc_psramb_tzsrg_r1_en` reader - "]
pub type TzcPsrambTzsrgR1EnR = crate::BitReader;
#[doc = "Field `tzc_psramb_tzsrg_r2_en` reader - "]
pub type TzcPsrambTzsrgR2EnR = crate::BitReader;
#[doc = "Field `tzc_psramb_tzsrg_r0_lock` reader - "]
pub type TzcPsrambTzsrgR0LockR = crate::BitReader;
#[doc = "Field `tzc_psramb_tzsrg_r1_lock` reader - "]
pub type TzcPsrambTzsrgR1LockR = crate::BitReader;
#[doc = "Field `tzc_psramb_tzsrg_r2_lock` reader - "]
pub type TzcPsrambTzsrgR2LockR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r0_id_en(&self) -> TzcPsrambTzsrgR0IdEnR {
        TzcPsrambTzsrgR0IdEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r1_id_en(&self) -> TzcPsrambTzsrgR1IdEnR {
        TzcPsrambTzsrgR1IdEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r2_id_en(&self) -> TzcPsrambTzsrgR2IdEnR {
        TzcPsrambTzsrgR2IdEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r0_en(&self) -> TzcPsrambTzsrgR0EnR {
        TzcPsrambTzsrgR0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r1_en(&self) -> TzcPsrambTzsrgR1EnR {
        TzcPsrambTzsrgR1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r2_en(&self) -> TzcPsrambTzsrgR2EnR {
        TzcPsrambTzsrgR2EnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r0_lock(&self) -> TzcPsrambTzsrgR0LockR {
        TzcPsrambTzsrgR0LockR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r1_lock(&self) -> TzcPsrambTzsrgR1LockR {
        TzcPsrambTzsrgR1LockR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r2_lock(&self) -> TzcPsrambTzsrgR2LockR {
        TzcPsrambTzsrgR2LockR::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "PSRAM-B TZSRG Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psramb_tzsrg_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcPsrambTzsrgCtrlSpec;
impl crate::RegisterSpec for TzcPsrambTzsrgCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_psramb_tzsrg_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcPsrambTzsrgCtrlSpec {}
#[doc = "`reset()` method sets TZC_PSRAMB_TZSRG_CTRL to value 0"]
impl crate::Resettable for TzcPsrambTzsrgCtrlSpec {}
