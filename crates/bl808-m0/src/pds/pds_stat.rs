#[doc = "Register `PDS_STAT` reader"]
pub type R = crate::R<PdsStatSpec>;
#[doc = "Register `PDS_STAT` writer"]
pub type W = crate::W<PdsStatSpec>;
#[doc = "Field `ro_pds_state` reader - "]
pub type RoPdsStateR = crate::FieldReader;
#[doc = "Field `ro_pds_rf_state` reader - "]
pub type RoPdsRfStateR = crate::FieldReader;
#[doc = "Field `pds_reset_event` reader - "]
pub type PdsResetEventR = crate::FieldReader;
#[doc = "Field `pds_clr_reset_event` reader - "]
pub type PdsClrResetEventR = crate::BitReader;
#[doc = "Field `pds_clr_reset_event` writer - "]
pub type PdsClrResetEventW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn ro_pds_state(&self) -> RoPdsStateR {
        RoPdsStateR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn ro_pds_rf_state(&self) -> RoPdsRfStateR {
        RoPdsRfStateR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pds_reset_event(&self) -> PdsResetEventR {
        PdsResetEventR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pds_clr_reset_event(&self) -> PdsClrResetEventR {
        PdsClrResetEventR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pds_clr_reset_event(&mut self) -> PdsClrResetEventW<'_, PdsStatSpec> {
        PdsClrResetEventW::new(self, 31)
    }
}
#[doc = "PDS Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdsStatSpec;
impl crate::RegisterSpec for PdsStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_stat::R`](R) reader structure"]
impl crate::Readable for PdsStatSpec {}
#[doc = "`write(|w| ..)` method takes [`pds_stat::W`](W) writer structure"]
impl crate::Writable for PdsStatSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x8000_0000;
}
#[doc = "`reset()` method sets PDS_STAT to value 0"]
impl crate::Resettable for PdsStatSpec {}
