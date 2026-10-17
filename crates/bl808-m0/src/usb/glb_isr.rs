#[doc = "Register `GLB_ISR` reader"]
pub type R = crate::R<GlbIsrSpec>;
#[doc = "Field `DEV_INT` reader - Device interrupt status"]
pub type DevIntR = crate::BitReader;
#[doc = "Field `OTG_INT` reader - OTG interrupt status"]
pub type OtgIntR = crate::BitReader;
#[doc = "Field `HC_INT` reader - Host Controller interrupt status"]
pub type HcIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Device interrupt status"]
    #[inline(always)]
    pub fn dev_int(&self) -> DevIntR {
        DevIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OTG interrupt status"]
    #[inline(always)]
    pub fn otg_int(&self) -> OtgIntR {
        OtgIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Host Controller interrupt status"]
    #[inline(always)]
    pub fn hc_int(&self) -> HcIntR {
        HcIntR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Global Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`glb_isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlbIsrSpec;
impl crate::RegisterSpec for GlbIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`glb_isr::R`](R) reader structure"]
impl crate::Readable for GlbIsrSpec {}
#[doc = "`reset()` method sets GLB_ISR to value 0"]
impl crate::Resettable for GlbIsrSpec {}
