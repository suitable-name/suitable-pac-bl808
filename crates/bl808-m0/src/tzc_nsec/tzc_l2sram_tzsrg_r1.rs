#[doc = "Register `TZC_L2SRAM_TZSRG_R1` reader"]
pub type R = crate::R<TzcL2sramTzsrgR1Spec>;
#[doc = "Field `tzc_l2sram_tzsrg_r1_end` reader - "]
pub type TzcL2sramTzsrgR1EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_l2sram_tzsrg_r1_start` reader - "]
pub type TzcL2sramTzsrgR1StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r1_end(&self) -> TzcL2sramTzsrgR1EndR {
        TzcL2sramTzsrgR1EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r1_start(&self) -> TzcL2sramTzsrgR1StartR {
        TzcL2sramTzsrgR1StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "L2SRAM TZSRG Region 1 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_l2sram_tzsrg_r1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcL2sramTzsrgR1Spec;
impl crate::RegisterSpec for TzcL2sramTzsrgR1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_l2sram_tzsrg_r1::R`](R) reader structure"]
impl crate::Readable for TzcL2sramTzsrgR1Spec {}
#[doc = "`reset()` method sets TZC_L2SRAM_TZSRG_R1 to value 0"]
impl crate::Resettable for TzcL2sramTzsrgR1Spec {}
