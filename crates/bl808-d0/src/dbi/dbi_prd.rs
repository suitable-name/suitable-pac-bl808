#[doc = "Register `DBI_PRD` reader"]
pub type R = crate::R<DbiPrdSpec>;
#[doc = "Register `DBI_PRD` writer"]
pub type W = crate::W<DbiPrdSpec>;
#[doc = "Field `cr_dbi_prd_s` reader - Start condition duration (cycles)"]
pub type CrDbiPrdSR = crate::FieldReader;
#[doc = "Field `cr_dbi_prd_s` writer - Start condition duration (cycles)"]
pub type CrDbiPrdSW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_dbi_prd_i` reader - Interval period duration (cycles)"]
pub type CrDbiPrdIR = crate::FieldReader;
#[doc = "Field `cr_dbi_prd_i` writer - Interval period duration (cycles)"]
pub type CrDbiPrdIW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_dbi_prd_d_ph_0` reader - Data phase 0 duration (cycles)"]
pub type CrDbiPrdDPh0R = crate::FieldReader;
#[doc = "Field `cr_dbi_prd_d_ph_0` writer - Data phase 0 duration (cycles)"]
pub type CrDbiPrdDPh0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_dbi_prd_d_ph_1` reader - Data phase 1 duration (cycles)"]
pub type CrDbiPrdDPh1R = crate::FieldReader;
#[doc = "Field `cr_dbi_prd_d_ph_1` writer - Data phase 1 duration (cycles)"]
pub type CrDbiPrdDPh1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Start condition duration (cycles)"]
    #[inline(always)]
    pub fn cr_dbi_prd_s(&self) -> CrDbiPrdSR {
        CrDbiPrdSR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interval period duration (cycles)"]
    #[inline(always)]
    pub fn cr_dbi_prd_i(&self) -> CrDbiPrdIR {
        CrDbiPrdIR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data phase 0 duration (cycles)"]
    #[inline(always)]
    pub fn cr_dbi_prd_d_ph_0(&self) -> CrDbiPrdDPh0R {
        CrDbiPrdDPh0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data phase 1 duration (cycles)"]
    #[inline(always)]
    pub fn cr_dbi_prd_d_ph_1(&self) -> CrDbiPrdDPh1R {
        CrDbiPrdDPh1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Start condition duration (cycles)"]
    #[inline(always)]
    pub fn cr_dbi_prd_s(&mut self) -> CrDbiPrdSW<'_, DbiPrdSpec> {
        CrDbiPrdSW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interval period duration (cycles)"]
    #[inline(always)]
    pub fn cr_dbi_prd_i(&mut self) -> CrDbiPrdIW<'_, DbiPrdSpec> {
        CrDbiPrdIW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Data phase 0 duration (cycles)"]
    #[inline(always)]
    pub fn cr_dbi_prd_d_ph_0(&mut self) -> CrDbiPrdDPh0W<'_, DbiPrdSpec> {
        CrDbiPrdDPh0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Data phase 1 duration (cycles)"]
    #[inline(always)]
    pub fn cr_dbi_prd_d_ph_1(&mut self) -> CrDbiPrdDPh1W<'_, DbiPrdSpec> {
        CrDbiPrdDPh1W::new(self, 24)
    }
}
#[doc = "DBI Timing Period Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dbi_prd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbi_prd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbiPrdSpec;
impl crate::RegisterSpec for DbiPrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbi_prd::R`](R) reader structure"]
impl crate::Readable for DbiPrdSpec {}
#[doc = "`write(|w| ..)` method takes [`dbi_prd::W`](W) writer structure"]
impl crate::Writable for DbiPrdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBI_PRD to value 0x0f0f_0f0f"]
impl crate::Resettable for DbiPrdSpec {
    const RESET_VALUE: u32 = 0x0f0f_0f0f;
}
