#[doc = "Register `TZC_PSRAMA_TZSRG_CTRL` reader"]
pub type R = crate::R<TzcPsramaTzsrgCtrlSpec>;
#[doc = "Field `tzc_psrama_tzsrg_r0_id_en` reader - "]
pub type TzcPsramaTzsrgR0IdEnR = crate::FieldReader;
#[doc = "Field `tzc_psrama_tzsrg_r1_id_en` reader - "]
pub type TzcPsramaTzsrgR1IdEnR = crate::FieldReader;
#[doc = "Field `tzc_psrama_tzsrg_r2_id_en` reader - "]
pub type TzcPsramaTzsrgR2IdEnR = crate::FieldReader;
#[doc = "Field `tzc_psrama_tzsrg_r0_en` reader - "]
pub type TzcPsramaTzsrgR0EnR = crate::BitReader;
#[doc = "Field `tzc_psrama_tzsrg_r1_en` reader - "]
pub type TzcPsramaTzsrgR1EnR = crate::BitReader;
#[doc = "Field `tzc_psrama_tzsrg_r2_en` reader - "]
pub type TzcPsramaTzsrgR2EnR = crate::BitReader;
#[doc = "Field `tzc_psrama_tzsrg_r0_lock` reader - "]
pub type TzcPsramaTzsrgR0LockR = crate::BitReader;
#[doc = "Field `tzc_psrama_tzsrg_r1_lock` reader - "]
pub type TzcPsramaTzsrgR1LockR = crate::BitReader;
#[doc = "Field `tzc_psrama_tzsrg_r2_lock` reader - "]
pub type TzcPsramaTzsrgR2LockR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r0_id_en(&self) -> TzcPsramaTzsrgR0IdEnR {
        TzcPsramaTzsrgR0IdEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r1_id_en(&self) -> TzcPsramaTzsrgR1IdEnR {
        TzcPsramaTzsrgR1IdEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r2_id_en(&self) -> TzcPsramaTzsrgR2IdEnR {
        TzcPsramaTzsrgR2IdEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r0_en(&self) -> TzcPsramaTzsrgR0EnR {
        TzcPsramaTzsrgR0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r1_en(&self) -> TzcPsramaTzsrgR1EnR {
        TzcPsramaTzsrgR1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r2_en(&self) -> TzcPsramaTzsrgR2EnR {
        TzcPsramaTzsrgR2EnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r0_lock(&self) -> TzcPsramaTzsrgR0LockR {
        TzcPsramaTzsrgR0LockR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r1_lock(&self) -> TzcPsramaTzsrgR1LockR {
        TzcPsramaTzsrgR1LockR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r2_lock(&self) -> TzcPsramaTzsrgR2LockR {
        TzcPsramaTzsrgR2LockR::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "PSRAM-A TZSRG Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psrama_tzsrg_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcPsramaTzsrgCtrlSpec;
impl crate::RegisterSpec for TzcPsramaTzsrgCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_psrama_tzsrg_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcPsramaTzsrgCtrlSpec {}
#[doc = "`reset()` method sets TZC_PSRAMA_TZSRG_CTRL to value 0"]
impl crate::Resettable for TzcPsramaTzsrgCtrlSpec {}
