#[doc = "Register `TZC_SF_TZSRG_MSB` reader"]
pub type R = crate::R<TzcSfTzsrgMsbSpec>;
#[doc = "Field `tzc_sf_tzsrg_r0_end_msb` reader - "]
pub type TzcSfTzsrgR0EndMsbR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r0_start_msb` reader - "]
pub type TzcSfTzsrgR0StartMsbR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r1_end_msb` reader - "]
pub type TzcSfTzsrgR1EndMsbR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r1_start_msb` reader - "]
pub type TzcSfTzsrgR1StartMsbR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r2_end_msb` reader - "]
pub type TzcSfTzsrgR2EndMsbR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r2_start_msb` reader - "]
pub type TzcSfTzsrgR2StartMsbR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r3_end_msb` reader - "]
pub type TzcSfTzsrgR3EndMsbR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r3_start_msb` reader - "]
pub type TzcSfTzsrgR3StartMsbR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r0_end_msb(&self) -> TzcSfTzsrgR0EndMsbR {
        TzcSfTzsrgR0EndMsbR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r0_start_msb(&self) -> TzcSfTzsrgR0StartMsbR {
        TzcSfTzsrgR0StartMsbR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_end_msb(&self) -> TzcSfTzsrgR1EndMsbR {
        TzcSfTzsrgR1EndMsbR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_start_msb(&self) -> TzcSfTzsrgR1StartMsbR {
        TzcSfTzsrgR1StartMsbR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_end_msb(&self) -> TzcSfTzsrgR2EndMsbR {
        TzcSfTzsrgR2EndMsbR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_start_msb(&self) -> TzcSfTzsrgR2StartMsbR {
        TzcSfTzsrgR2StartMsbR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_end_msb(&self) -> TzcSfTzsrgR3EndMsbR {
        TzcSfTzsrgR3EndMsbR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_start_msb(&self) -> TzcSfTzsrgR3StartMsbR {
        TzcSfTzsrgR3StartMsbR::new(((self.bits >> 28) & 7) as u8)
    }
}
#[doc = "SF TZSRG Address MSB Configuration (Bits \\[21:19\\] for 64M Flash)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_msb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcSfTzsrgMsbSpec;
impl crate::RegisterSpec for TzcSfTzsrgMsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_sf_tzsrg_msb::R`](R) reader structure"]
impl crate::Readable for TzcSfTzsrgMsbSpec {}
#[doc = "`reset()` method sets TZC_SF_TZSRG_MSB to value 0"]
impl crate::Resettable for TzcSfTzsrgMsbSpec {}
