#[doc = "Register `coex_control_2` reader"]
pub type R = crate::R<CoexControl2Spec>;
#[doc = "Register `coex_control_2` writer"]
pub type W = crate::W<CoexControl2Spec>;
#[doc = "Field `force_bk` reader - Force grant for Background (BK) traffic class"]
pub type ForceBkR = crate::BitReader;
#[doc = "Field `force_bk` writer - Force grant for Background (BK) traffic class"]
pub type ForceBkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `force_be` reader - Force grant for Best Effort (BE) traffic class"]
pub type ForceBeR = crate::BitReader;
#[doc = "Field `force_be` writer - Force grant for Best Effort (BE) traffic class"]
pub type ForceBeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `force_vi` reader - Force grant for Video (VI) traffic class"]
pub type ForceViR = crate::BitReader;
#[doc = "Field `force_vi` writer - Force grant for Video (VI) traffic class"]
pub type ForceViW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `force_vo` reader - Force grant for Voice (VO) traffic class"]
pub type ForceVoR = crate::BitReader;
#[doc = "Field `force_vo` writer - Force grant for Voice (VO) traffic class"]
pub type ForceVoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `force_bcn` reader - Force grant for Beacon (BCN) traffic class"]
pub type ForceBcnR = crate::BitReader;
#[doc = "Field `force_bcn` writer - Force grant for Beacon (BCN) traffic class"]
pub type ForceBcnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Force grant for Background (BK) traffic class"]
    #[inline(always)]
    pub fn force_bk(&self) -> ForceBkR {
        ForceBkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force grant for Best Effort (BE) traffic class"]
    #[inline(always)]
    pub fn force_be(&self) -> ForceBeR {
        ForceBeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force grant for Video (VI) traffic class"]
    #[inline(always)]
    pub fn force_vi(&self) -> ForceViR {
        ForceViR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force grant for Voice (VO) traffic class"]
    #[inline(always)]
    pub fn force_vo(&self) -> ForceVoR {
        ForceVoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force grant for Beacon (BCN) traffic class"]
    #[inline(always)]
    pub fn force_bcn(&self) -> ForceBcnR {
        ForceBcnR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force grant for Background (BK) traffic class"]
    #[inline(always)]
    pub fn force_bk(&mut self) -> ForceBkW<'_, CoexControl2Spec> {
        ForceBkW::new(self, 0)
    }
    #[doc = "Bit 1 - Force grant for Best Effort (BE) traffic class"]
    #[inline(always)]
    pub fn force_be(&mut self) -> ForceBeW<'_, CoexControl2Spec> {
        ForceBeW::new(self, 1)
    }
    #[doc = "Bit 2 - Force grant for Video (VI) traffic class"]
    #[inline(always)]
    pub fn force_vi(&mut self) -> ForceViW<'_, CoexControl2Spec> {
        ForceViW::new(self, 2)
    }
    #[doc = "Bit 3 - Force grant for Voice (VO) traffic class"]
    #[inline(always)]
    pub fn force_vo(&mut self) -> ForceVoW<'_, CoexControl2Spec> {
        ForceVoW::new(self, 3)
    }
    #[doc = "Bit 4 - Force grant for Beacon (BCN) traffic class"]
    #[inline(always)]
    pub fn force_bcn(&mut self) -> ForceBcnW<'_, CoexControl2Spec> {
        ForceBcnW::new(self, 4)
    }
}
#[doc = "Coexistence Control Register 2 (Traffic Class Force Grants)\n\nYou can [`read`](crate::Reg::read) this register and get [`coex_control_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`coex_control_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoexControl2Spec;
impl crate::RegisterSpec for CoexControl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`coex_control_2::R`](R) reader structure"]
impl crate::Readable for CoexControl2Spec {}
#[doc = "`write(|w| ..)` method takes [`coex_control_2::W`](W) writer structure"]
impl crate::Writable for CoexControl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets coex_control_2 to value 0"]
impl crate::Resettable for CoexControl2Spec {}
