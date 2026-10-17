#[doc = "Register `TZC_L2SRAM_TZSRG_R2` reader"]
pub type R = crate::R<TzcL2sramTzsrgR2Spec>;
#[doc = "Field `tzc_l2sram_tzsrg_r2_end` reader - "]
pub type TzcL2sramTzsrgR2EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_l2sram_tzsrg_r2_start` reader - "]
pub type TzcL2sramTzsrgR2StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r2_end(&self) -> TzcL2sramTzsrgR2EndR {
        TzcL2sramTzsrgR2EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r2_start(&self) -> TzcL2sramTzsrgR2StartR {
        TzcL2sramTzsrgR2StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "L2SRAM TZSRG Region 2 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_l2sram_tzsrg_r2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcL2sramTzsrgR2Spec;
impl crate::RegisterSpec for TzcL2sramTzsrgR2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_l2sram_tzsrg_r2::R`](R) reader structure"]
impl crate::Readable for TzcL2sramTzsrgR2Spec {}
#[doc = "`reset()` method sets TZC_L2SRAM_TZSRG_R2 to value 0"]
impl crate::Resettable for TzcL2sramTzsrgR2Spec {}
