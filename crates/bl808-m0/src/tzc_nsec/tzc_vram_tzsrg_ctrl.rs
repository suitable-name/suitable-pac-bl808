#[doc = "Register `TZC_VRAM_TZSRG_CTRL` reader"]
pub type R = crate::R<TzcVramTzsrgCtrlSpec>;
#[doc = "Field `tzc_vram_tzsrg_r0_id_en` reader - "]
pub type TzcVramTzsrgR0IdEnR = crate::FieldReader;
#[doc = "Field `tzc_vram_tzsrg_r1_id_en` reader - "]
pub type TzcVramTzsrgR1IdEnR = crate::FieldReader;
#[doc = "Field `tzc_vram_tzsrg_r2_id_en` reader - "]
pub type TzcVramTzsrgR2IdEnR = crate::FieldReader;
#[doc = "Field `tzc_vram_tzsrg_r0_en` reader - "]
pub type TzcVramTzsrgR0EnR = crate::BitReader;
#[doc = "Field `tzc_vram_tzsrg_r1_en` reader - "]
pub type TzcVramTzsrgR1EnR = crate::BitReader;
#[doc = "Field `tzc_vram_tzsrg_r2_en` reader - "]
pub type TzcVramTzsrgR2EnR = crate::BitReader;
#[doc = "Field `tzc_vram_tzsrg_r0_lock` reader - "]
pub type TzcVramTzsrgR0LockR = crate::BitReader;
#[doc = "Field `tzc_vram_tzsrg_r1_lock` reader - "]
pub type TzcVramTzsrgR1LockR = crate::BitReader;
#[doc = "Field `tzc_vram_tzsrg_r2_lock` reader - "]
pub type TzcVramTzsrgR2LockR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r0_id_en(&self) -> TzcVramTzsrgR0IdEnR {
        TzcVramTzsrgR0IdEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r1_id_en(&self) -> TzcVramTzsrgR1IdEnR {
        TzcVramTzsrgR1IdEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r2_id_en(&self) -> TzcVramTzsrgR2IdEnR {
        TzcVramTzsrgR2IdEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r0_en(&self) -> TzcVramTzsrgR0EnR {
        TzcVramTzsrgR0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r1_en(&self) -> TzcVramTzsrgR1EnR {
        TzcVramTzsrgR1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r2_en(&self) -> TzcVramTzsrgR2EnR {
        TzcVramTzsrgR2EnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r0_lock(&self) -> TzcVramTzsrgR0LockR {
        TzcVramTzsrgR0LockR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r1_lock(&self) -> TzcVramTzsrgR1LockR {
        TzcVramTzsrgR1LockR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r2_lock(&self) -> TzcVramTzsrgR2LockR {
        TzcVramTzsrgR2LockR::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "VRAM TZSRG Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_vram_tzsrg_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcVramTzsrgCtrlSpec;
impl crate::RegisterSpec for TzcVramTzsrgCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_vram_tzsrg_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcVramTzsrgCtrlSpec {}
#[doc = "`reset()` method sets TZC_VRAM_TZSRG_CTRL to value 0"]
impl crate::Resettable for TzcVramTzsrgCtrlSpec {}
