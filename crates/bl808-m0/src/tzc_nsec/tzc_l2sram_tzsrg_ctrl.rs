#[doc = "Register `TZC_L2SRAM_TZSRG_CTRL` reader"]
pub type R = crate::R<TzcL2sramTzsrgCtrlSpec>;
#[doc = "Field `tzc_l2sram_tzsrg_r0_id_en` reader - "]
pub type TzcL2sramTzsrgR0IdEnR = crate::FieldReader;
#[doc = "Field `tzc_l2sram_tzsrg_r1_id_en` reader - "]
pub type TzcL2sramTzsrgR1IdEnR = crate::FieldReader;
#[doc = "Field `tzc_l2sram_tzsrg_r2_id_en` reader - "]
pub type TzcL2sramTzsrgR2IdEnR = crate::FieldReader;
#[doc = "Field `tzc_l2sram_tzsrg_r0_en` reader - "]
pub type TzcL2sramTzsrgR0EnR = crate::BitReader;
#[doc = "Field `tzc_l2sram_tzsrg_r1_en` reader - "]
pub type TzcL2sramTzsrgR1EnR = crate::BitReader;
#[doc = "Field `tzc_l2sram_tzsrg_r2_en` reader - "]
pub type TzcL2sramTzsrgR2EnR = crate::BitReader;
#[doc = "Field `tzc_l2sram_tzsrg_r0_lock` reader - "]
pub type TzcL2sramTzsrgR0LockR = crate::BitReader;
#[doc = "Field `tzc_l2sram_tzsrg_r1_lock` reader - "]
pub type TzcL2sramTzsrgR1LockR = crate::BitReader;
#[doc = "Field `tzc_l2sram_tzsrg_r2_lock` reader - "]
pub type TzcL2sramTzsrgR2LockR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r0_id_en(&self) -> TzcL2sramTzsrgR0IdEnR {
        TzcL2sramTzsrgR0IdEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r1_id_en(&self) -> TzcL2sramTzsrgR1IdEnR {
        TzcL2sramTzsrgR1IdEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r2_id_en(&self) -> TzcL2sramTzsrgR2IdEnR {
        TzcL2sramTzsrgR2IdEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r0_en(&self) -> TzcL2sramTzsrgR0EnR {
        TzcL2sramTzsrgR0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r1_en(&self) -> TzcL2sramTzsrgR1EnR {
        TzcL2sramTzsrgR1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r2_en(&self) -> TzcL2sramTzsrgR2EnR {
        TzcL2sramTzsrgR2EnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r0_lock(&self) -> TzcL2sramTzsrgR0LockR {
        TzcL2sramTzsrgR0LockR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r1_lock(&self) -> TzcL2sramTzsrgR1LockR {
        TzcL2sramTzsrgR1LockR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r2_lock(&self) -> TzcL2sramTzsrgR2LockR {
        TzcL2sramTzsrgR2LockR::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "L2SRAM TZSRG Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_l2sram_tzsrg_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcL2sramTzsrgCtrlSpec;
impl crate::RegisterSpec for TzcL2sramTzsrgCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_l2sram_tzsrg_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcL2sramTzsrgCtrlSpec {}
#[doc = "`reset()` method sets TZC_L2SRAM_TZSRG_CTRL to value 0"]
impl crate::Resettable for TzcL2sramTzsrgCtrlSpec {}
