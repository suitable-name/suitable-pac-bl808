#[doc = "Register `TZC_XRAM_TZSRG_CTRL` reader"]
pub type R = crate::R<TzcXramTzsrgCtrlSpec>;
#[doc = "Field `tzc_xram_tzsrg_r0_id_en` reader - "]
pub type TzcXramTzsrgR0IdEnR = crate::FieldReader;
#[doc = "Field `tzc_xram_tzsrg_r1_id_en` reader - "]
pub type TzcXramTzsrgR1IdEnR = crate::FieldReader;
#[doc = "Field `tzc_xram_tzsrg_r2_id_en` reader - "]
pub type TzcXramTzsrgR2IdEnR = crate::FieldReader;
#[doc = "Field `tzc_xram_tzsrg_r0_en` reader - "]
pub type TzcXramTzsrgR0EnR = crate::BitReader;
#[doc = "Field `tzc_xram_tzsrg_r1_en` reader - "]
pub type TzcXramTzsrgR1EnR = crate::BitReader;
#[doc = "Field `tzc_xram_tzsrg_r2_en` reader - "]
pub type TzcXramTzsrgR2EnR = crate::BitReader;
#[doc = "Field `tzc_xram_tzsrg_r0_lock` reader - "]
pub type TzcXramTzsrgR0LockR = crate::BitReader;
#[doc = "Field `tzc_xram_tzsrg_r1_lock` reader - "]
pub type TzcXramTzsrgR1LockR = crate::BitReader;
#[doc = "Field `tzc_xram_tzsrg_r2_lock` reader - "]
pub type TzcXramTzsrgR2LockR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r0_id_en(&self) -> TzcXramTzsrgR0IdEnR {
        TzcXramTzsrgR0IdEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r1_id_en(&self) -> TzcXramTzsrgR1IdEnR {
        TzcXramTzsrgR1IdEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r2_id_en(&self) -> TzcXramTzsrgR2IdEnR {
        TzcXramTzsrgR2IdEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r0_en(&self) -> TzcXramTzsrgR0EnR {
        TzcXramTzsrgR0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r1_en(&self) -> TzcXramTzsrgR1EnR {
        TzcXramTzsrgR1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r2_en(&self) -> TzcXramTzsrgR2EnR {
        TzcXramTzsrgR2EnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r0_lock(&self) -> TzcXramTzsrgR0LockR {
        TzcXramTzsrgR0LockR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r1_lock(&self) -> TzcXramTzsrgR1LockR {
        TzcXramTzsrgR1LockR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r2_lock(&self) -> TzcXramTzsrgR2LockR {
        TzcXramTzsrgR2LockR::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "XRAM TZSRG Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_xram_tzsrg_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcXramTzsrgCtrlSpec;
impl crate::RegisterSpec for TzcXramTzsrgCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_xram_tzsrg_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcXramTzsrgCtrlSpec {}
#[doc = "`reset()` method sets TZC_XRAM_TZSRG_CTRL to value 0"]
impl crate::Resettable for TzcXramTzsrgCtrlSpec {}
