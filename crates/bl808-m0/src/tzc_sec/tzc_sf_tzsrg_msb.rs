#[doc = "Register `TZC_SF_TZSRG_MSB` reader"]
pub type R = crate::R<TzcSfTzsrgMsbSpec>;
#[doc = "Register `TZC_SF_TZSRG_MSB` writer"]
pub type W = crate::W<TzcSfTzsrgMsbSpec>;
#[doc = "Field `tzc_sf_tzsrg_r0_end_msb` reader - Region 0 end address MSBs"]
pub type TzcSfTzsrgR0EndMsbR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r0_end_msb` writer - Region 0 end address MSBs"]
pub type TzcSfTzsrgR0EndMsbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tzc_sf_tzsrg_r0_start_msb` reader - Region 0 start address MSBs"]
pub type TzcSfTzsrgR0StartMsbR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r0_start_msb` writer - Region 0 start address MSBs"]
pub type TzcSfTzsrgR0StartMsbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tzc_sf_tzsrg_r1_end_msb` reader - Region 1 end address MSBs"]
pub type TzcSfTzsrgR1EndMsbR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r1_end_msb` writer - Region 1 end address MSBs"]
pub type TzcSfTzsrgR1EndMsbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tzc_sf_tzsrg_r1_start_msb` reader - Region 1 start address MSBs"]
pub type TzcSfTzsrgR1StartMsbR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r1_start_msb` writer - Region 1 start address MSBs"]
pub type TzcSfTzsrgR1StartMsbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tzc_sf_tzsrg_r2_end_msb` reader - Region 2 end address MSBs"]
pub type TzcSfTzsrgR2EndMsbR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r2_end_msb` writer - Region 2 end address MSBs"]
pub type TzcSfTzsrgR2EndMsbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tzc_sf_tzsrg_r2_start_msb` reader - Region 2 start address MSBs"]
pub type TzcSfTzsrgR2StartMsbR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r2_start_msb` writer - Region 2 start address MSBs"]
pub type TzcSfTzsrgR2StartMsbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tzc_sf_tzsrg_r3_end_msb` reader - Region 3 end address MSBs"]
pub type TzcSfTzsrgR3EndMsbR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r3_end_msb` writer - Region 3 end address MSBs"]
pub type TzcSfTzsrgR3EndMsbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tzc_sf_tzsrg_r3_start_msb` reader - Region 3 start address MSBs"]
pub type TzcSfTzsrgR3StartMsbR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r3_start_msb` writer - Region 3 start address MSBs"]
pub type TzcSfTzsrgR3StartMsbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Region 0 end address MSBs"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r0_end_msb(&self) -> TzcSfTzsrgR0EndMsbR {
        TzcSfTzsrgR0EndMsbR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Region 0 start address MSBs"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r0_start_msb(&self) -> TzcSfTzsrgR0StartMsbR {
        TzcSfTzsrgR0StartMsbR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Region 1 end address MSBs"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_end_msb(&self) -> TzcSfTzsrgR1EndMsbR {
        TzcSfTzsrgR1EndMsbR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Region 1 start address MSBs"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_start_msb(&self) -> TzcSfTzsrgR1StartMsbR {
        TzcSfTzsrgR1StartMsbR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Region 2 end address MSBs"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_end_msb(&self) -> TzcSfTzsrgR2EndMsbR {
        TzcSfTzsrgR2EndMsbR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Region 2 start address MSBs"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_start_msb(&self) -> TzcSfTzsrgR2StartMsbR {
        TzcSfTzsrgR2StartMsbR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Region 3 end address MSBs"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_end_msb(&self) -> TzcSfTzsrgR3EndMsbR {
        TzcSfTzsrgR3EndMsbR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Region 3 start address MSBs"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_start_msb(&self) -> TzcSfTzsrgR3StartMsbR {
        TzcSfTzsrgR3StartMsbR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Region 0 end address MSBs"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r0_end_msb(&mut self) -> TzcSfTzsrgR0EndMsbW<'_, TzcSfTzsrgMsbSpec> {
        TzcSfTzsrgR0EndMsbW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Region 0 start address MSBs"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r0_start_msb(&mut self) -> TzcSfTzsrgR0StartMsbW<'_, TzcSfTzsrgMsbSpec> {
        TzcSfTzsrgR0StartMsbW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Region 1 end address MSBs"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_end_msb(&mut self) -> TzcSfTzsrgR1EndMsbW<'_, TzcSfTzsrgMsbSpec> {
        TzcSfTzsrgR1EndMsbW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Region 1 start address MSBs"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_start_msb(&mut self) -> TzcSfTzsrgR1StartMsbW<'_, TzcSfTzsrgMsbSpec> {
        TzcSfTzsrgR1StartMsbW::new(self, 12)
    }
    #[doc = "Bits 16:18 - Region 2 end address MSBs"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_end_msb(&mut self) -> TzcSfTzsrgR2EndMsbW<'_, TzcSfTzsrgMsbSpec> {
        TzcSfTzsrgR2EndMsbW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Region 2 start address MSBs"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_start_msb(&mut self) -> TzcSfTzsrgR2StartMsbW<'_, TzcSfTzsrgMsbSpec> {
        TzcSfTzsrgR2StartMsbW::new(self, 20)
    }
    #[doc = "Bits 24:26 - Region 3 end address MSBs"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_end_msb(&mut self) -> TzcSfTzsrgR3EndMsbW<'_, TzcSfTzsrgMsbSpec> {
        TzcSfTzsrgR3EndMsbW::new(self, 24)
    }
    #[doc = "Bits 28:30 - Region 3 start address MSBs"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_start_msb(&mut self) -> TzcSfTzsrgR3StartMsbW<'_, TzcSfTzsrgMsbSpec> {
        TzcSfTzsrgR3StartMsbW::new(self, 28)
    }
}
#[doc = "Serial Flash Security Region Address MSBs\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_sf_tzsrg_msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcSfTzsrgMsbSpec;
impl crate::RegisterSpec for TzcSfTzsrgMsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_sf_tzsrg_msb::R`](R) reader structure"]
impl crate::Readable for TzcSfTzsrgMsbSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_sf_tzsrg_msb::W`](W) writer structure"]
impl crate::Writable for TzcSfTzsrgMsbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_SF_TZSRG_MSB to value 0"]
impl crate::Resettable for TzcSfTzsrgMsbSpec {}
