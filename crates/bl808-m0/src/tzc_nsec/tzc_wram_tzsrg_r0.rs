#[doc = "Register `TZC_WRAM_TZSRG_R0` reader"]
pub type R = crate::R<TzcWramTzsrgR0Spec>;
#[doc = "Field `tzc_wram_tzsrg_r0_end` reader - "]
pub type TzcWramTzsrgR0EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_wram_tzsrg_r0_start` reader - "]
pub type TzcWramTzsrgR0StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r0_end(&self) -> TzcWramTzsrgR0EndR {
        TzcWramTzsrgR0EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r0_start(&self) -> TzcWramTzsrgR0StartR {
        TzcWramTzsrgR0StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "WRAM TZSRG Region 0 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_wram_tzsrg_r0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcWramTzsrgR0Spec;
impl crate::RegisterSpec for TzcWramTzsrgR0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_wram_tzsrg_r0::R`](R) reader structure"]
impl crate::Readable for TzcWramTzsrgR0Spec {}
#[doc = "`reset()` method sets TZC_WRAM_TZSRG_R0 to value 0"]
impl crate::Resettable for TzcWramTzsrgR0Spec {}
