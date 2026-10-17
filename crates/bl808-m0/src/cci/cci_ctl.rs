#[doc = "Register `CCI_CTL` reader"]
pub type R = crate::R<CciCtlSpec>;
#[doc = "Field `cci_write_flag` reader - Write operation busy flag"]
pub type CciWriteFlagR = crate::BitReader;
#[doc = "Field `cci_read_flag` reader - Read operation busy flag"]
pub type CciReadFlagR = crate::BitReader;
#[doc = "Field `ahb_state` reader - AHB state machine status"]
pub type AhbStateR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Write operation busy flag"]
    #[inline(always)]
    pub fn cci_write_flag(&self) -> CciWriteFlagR {
        CciWriteFlagR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read operation busy flag"]
    #[inline(always)]
    pub fn cci_read_flag(&self) -> CciReadFlagR {
        CciReadFlagR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - AHB state machine status"]
    #[inline(always)]
    pub fn ahb_state(&self) -> AhbStateR {
        AhbStateR::new(((self.bits >> 2) & 3) as u8)
    }
}
#[doc = "CCI Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`cci_ctl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CciCtlSpec;
impl crate::RegisterSpec for CciCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cci_ctl::R`](R) reader structure"]
impl crate::Readable for CciCtlSpec {}
#[doc = "`reset()` method sets CCI_CTL to value 0"]
impl crate::Resettable for CciCtlSpec {}
