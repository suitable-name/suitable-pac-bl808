#[doc = "Register `MODE` reader"]
pub type R = crate::R<ModeSpec>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<ModeSpec>;
#[doc = "Field `reset_request` reader - Request reset mode"]
pub type ResetRequestR = crate::BitReader;
#[doc = "Field `reset_request` writer - Request reset mode"]
pub type ResetRequestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `listen_only` reader - Listen Only Mode enable"]
pub type ListenOnlyR = crate::BitReader;
#[doc = "Field `listen_only` writer - Listen Only Mode enable"]
pub type ListenOnlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `selt_test` reader - Self Test Mode enable (e.g., Loopback)"]
pub type SeltTestR = crate::BitReader;
#[doc = "Field `selt_test` writer - Self Test Mode enable (e.g., Loopback)"]
pub type SeltTestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `acceptance_filter` reader - Acceptance Filter enable"]
pub type AcceptanceFilterR = crate::BitReader;
#[doc = "Field `acceptance_filter` writer - Acceptance Filter enable"]
pub type AcceptanceFilterW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Request reset mode"]
    #[inline(always)]
    pub fn reset_request(&self) -> ResetRequestR {
        ResetRequestR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Listen Only Mode enable"]
    #[inline(always)]
    pub fn listen_only(&self) -> ListenOnlyR {
        ListenOnlyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Self Test Mode enable (e.g., Loopback)"]
    #[inline(always)]
    pub fn selt_test(&self) -> SeltTestR {
        SeltTestR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Acceptance Filter enable"]
    #[inline(always)]
    pub fn acceptance_filter(&self) -> AcceptanceFilterR {
        AcceptanceFilterR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Request reset mode"]
    #[inline(always)]
    pub fn reset_request(&mut self) -> ResetRequestW<'_, ModeSpec> {
        ResetRequestW::new(self, 0)
    }
    #[doc = "Bit 1 - Listen Only Mode enable"]
    #[inline(always)]
    pub fn listen_only(&mut self) -> ListenOnlyW<'_, ModeSpec> {
        ListenOnlyW::new(self, 1)
    }
    #[doc = "Bit 2 - Self Test Mode enable (e.g., Loopback)"]
    #[inline(always)]
    pub fn selt_test(&mut self) -> SeltTestW<'_, ModeSpec> {
        SeltTestW::new(self, 2)
    }
    #[doc = "Bit 3 - Acceptance Filter enable"]
    #[inline(always)]
    pub fn acceptance_filter(&mut self) -> AcceptanceFilterW<'_, ModeSpec> {
        AcceptanceFilterW::new(self, 3)
    }
}
#[doc = "Operating Mode and Reset Mode Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeSpec;
impl crate::RegisterSpec for ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for ModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for ModeSpec {}
