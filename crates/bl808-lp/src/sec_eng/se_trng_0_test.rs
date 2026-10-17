#[doc = "Register `SE_TRNG_0_TEST` reader"]
pub type R = crate::R<SeTrng0TestSpec>;
#[doc = "Register `SE_TRNG_0_TEST` writer"]
pub type W = crate::W<SeTrng0TestSpec>;
#[doc = "Field `se_trng_0_test_en` reader - "]
pub type SeTrng0TestEnR = crate::BitReader;
#[doc = "Field `se_trng_0_test_en` writer - "]
pub type SeTrng0TestEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_trng_0_cp_test_en` reader - "]
pub type SeTrng0CpTestEnR = crate::BitReader;
#[doc = "Field `se_trng_0_cp_test_en` writer - "]
pub type SeTrng0CpTestEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_trng_0_cp_bypass` reader - "]
pub type SeTrng0CpBypassR = crate::BitReader;
#[doc = "Field `se_trng_0_cp_bypass` writer - "]
pub type SeTrng0CpBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_trng_0_ht_dis` reader - "]
pub type SeTrng0HtDisR = crate::BitReader;
#[doc = "Field `se_trng_0_ht_dis` writer - "]
pub type SeTrng0HtDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_trng_0_ht_alarm_n` reader - "]
pub type SeTrng0HtAlarmNR = crate::FieldReader;
#[doc = "Field `se_trng_0_ht_alarm_n` writer - "]
pub type SeTrng0HtAlarmNW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_trng_0_test_en(&self) -> SeTrng0TestEnR {
        SeTrng0TestEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_trng_0_cp_test_en(&self) -> SeTrng0CpTestEnR {
        SeTrng0CpTestEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_trng_0_cp_bypass(&self) -> SeTrng0CpBypassR {
        SeTrng0CpBypassR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_trng_0_ht_dis(&self) -> SeTrng0HtDisR {
        SeTrng0HtDisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn se_trng_0_ht_alarm_n(&self) -> SeTrng0HtAlarmNR {
        SeTrng0HtAlarmNR::new(((self.bits >> 4) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_trng_0_test_en(&mut self) -> SeTrng0TestEnW<'_, SeTrng0TestSpec> {
        SeTrng0TestEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_trng_0_cp_test_en(&mut self) -> SeTrng0CpTestEnW<'_, SeTrng0TestSpec> {
        SeTrng0CpTestEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_trng_0_cp_bypass(&mut self) -> SeTrng0CpBypassW<'_, SeTrng0TestSpec> {
        SeTrng0CpBypassW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_trng_0_ht_dis(&mut self) -> SeTrng0HtDisW<'_, SeTrng0TestSpec> {
        SeTrng0HtDisW::new(self, 3)
    }
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn se_trng_0_ht_alarm_n(&mut self) -> SeTrng0HtAlarmNW<'_, SeTrng0TestSpec> {
        SeTrng0HtAlarmNW::new(self, 4)
    }
}
#[doc = "TRNG Test Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_trng_0_test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeTrng0TestSpec;
impl crate::RegisterSpec for SeTrng0TestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_trng_0_test::R`](R) reader structure"]
impl crate::Readable for SeTrng0TestSpec {}
#[doc = "`write(|w| ..)` method takes [`se_trng_0_test::W`](W) writer structure"]
impl crate::Writable for SeTrng0TestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_TRNG_0_TEST to value 0"]
impl crate::Resettable for SeTrng0TestSpec {}
