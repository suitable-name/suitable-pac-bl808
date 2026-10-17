#[doc = "Register `TZC_WRAM_TZSRG_R1` reader"]
pub type R = crate::R<TzcWramTzsrgR1Spec>;
#[doc = "Field `tzc_wram_tzsrg_r1_end` reader - "]
pub type TzcWramTzsrgR1EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_wram_tzsrg_r1_start` reader - "]
pub type TzcWramTzsrgR1StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r1_end(&self) -> TzcWramTzsrgR1EndR {
        TzcWramTzsrgR1EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r1_start(&self) -> TzcWramTzsrgR1StartR {
        TzcWramTzsrgR1StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "WRAM TZSRG Region 1 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_wram_tzsrg_r1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcWramTzsrgR1Spec;
impl crate::RegisterSpec for TzcWramTzsrgR1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_wram_tzsrg_r1::R`](R) reader structure"]
impl crate::Readable for TzcWramTzsrgR1Spec {}
#[doc = "`reset()` method sets TZC_WRAM_TZSRG_R1 to value 0"]
impl crate::Resettable for TzcWramTzsrgR1Spec {}
