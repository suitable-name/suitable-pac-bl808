#[doc = "Register `TZC_L2SRAM_TZSRG_R0` reader"]
pub type R = crate::R<TzcL2sramTzsrgR0Spec>;
#[doc = "Field `tzc_l2sram_tzsrg_r0_end` reader - "]
pub type TzcL2sramTzsrgR0EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_l2sram_tzsrg_r0_start` reader - "]
pub type TzcL2sramTzsrgR0StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r0_end(&self) -> TzcL2sramTzsrgR0EndR {
        TzcL2sramTzsrgR0EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r0_start(&self) -> TzcL2sramTzsrgR0StartR {
        TzcL2sramTzsrgR0StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "L2SRAM TZSRG Region 0 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_l2sram_tzsrg_r0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcL2sramTzsrgR0Spec;
impl crate::RegisterSpec for TzcL2sramTzsrgR0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_l2sram_tzsrg_r0::R`](R) reader structure"]
impl crate::Readable for TzcL2sramTzsrgR0Spec {}
#[doc = "`reset()` method sets TZC_L2SRAM_TZSRG_R0 to value 0"]
impl crate::Resettable for TzcL2sramTzsrgR0Spec {}
