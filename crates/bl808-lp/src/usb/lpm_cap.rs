#[doc = "Register `LPM_CAP` reader"]
pub type R = crate::R<LpmCapSpec>;
#[doc = "Register `LPM_CAP` writer"]
pub type W = crate::W<LpmCapSpec>;
#[doc = "Field `LPM_WAKEUP_EN` reader - "]
pub type LpmWakeupEnR = crate::BitReader;
#[doc = "Field `LPM_WAKEUP_EN` writer - "]
pub type LpmWakeupEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lpm_wakeup_en(&self) -> LpmWakeupEnR {
        LpmWakeupEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lpm_wakeup_en(&mut self) -> LpmWakeupEnW<'_, LpmCapSpec> {
        LpmWakeupEnW::new(self, 0)
    }
}
#[doc = "LPM Capability Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpm_cap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpm_cap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpmCapSpec;
impl crate::RegisterSpec for LpmCapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpm_cap::R`](R) reader structure"]
impl crate::Readable for LpmCapSpec {}
#[doc = "`write(|w| ..)` method takes [`lpm_cap::W`](W) writer structure"]
impl crate::Writable for LpmCapSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPM_CAP to value 0"]
impl crate::Resettable for LpmCapSpec {}
