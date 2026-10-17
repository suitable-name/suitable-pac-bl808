#[doc = "Register `coex_pti` reader"]
pub type R = crate::R<CoexPtiSpec>;
#[doc = "Register `coex_pti` writer"]
pub type W = crate::W<CoexPtiSpec>;
#[doc = "Field `pti_ack` reader - ACK packet traffic priority level"]
pub type PtiAckR = crate::FieldReader;
#[doc = "Field `pti_ack` writer - ACK packet traffic priority level"]
pub type PtiAckW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pti_cntrl` reader - Control packet traffic priority level"]
pub type PtiCntrlR = crate::FieldReader;
#[doc = "Field `pti_cntrl` writer - Control packet traffic priority level"]
pub type PtiCntrlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pti_mgt` reader - Management packet traffic priority level"]
pub type PtiMgtR = crate::FieldReader;
#[doc = "Field `pti_mgt` writer - Management packet traffic priority level"]
pub type PtiMgtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pti_vo` reader - Voice (VO) packet traffic priority level"]
pub type PtiVoR = crate::FieldReader;
#[doc = "Field `pti_vo` writer - Voice (VO) packet traffic priority level"]
pub type PtiVoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pti_vi` reader - Video (VI) packet traffic priority level"]
pub type PtiViR = crate::FieldReader;
#[doc = "Field `pti_vi` writer - Video (VI) packet traffic priority level"]
pub type PtiViW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pti_be` reader - Best Effort (BE) packet traffic priority level"]
pub type PtiBeR = crate::FieldReader;
#[doc = "Field `pti_be` writer - Best Effort (BE) packet traffic priority level"]
pub type PtiBeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pti_bk` reader - Background (BK) packet traffic priority level"]
pub type PtiBkR = crate::FieldReader;
#[doc = "Field `pti_bk` writer - Background (BK) packet traffic priority level"]
pub type PtiBkW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pti_bcn` reader - Beacon (BCN) packet traffic priority level"]
pub type PtiBcnR = crate::FieldReader;
#[doc = "Field `pti_bcn` writer - Beacon (BCN) packet traffic priority level"]
pub type PtiBcnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ACK packet traffic priority level"]
    #[inline(always)]
    pub fn pti_ack(&self) -> PtiAckR {
        PtiAckR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Control packet traffic priority level"]
    #[inline(always)]
    pub fn pti_cntrl(&self) -> PtiCntrlR {
        PtiCntrlR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Management packet traffic priority level"]
    #[inline(always)]
    pub fn pti_mgt(&self) -> PtiMgtR {
        PtiMgtR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Voice (VO) packet traffic priority level"]
    #[inline(always)]
    pub fn pti_vo(&self) -> PtiVoR {
        PtiVoR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Video (VI) packet traffic priority level"]
    #[inline(always)]
    pub fn pti_vi(&self) -> PtiViR {
        PtiViR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Best Effort (BE) packet traffic priority level"]
    #[inline(always)]
    pub fn pti_be(&self) -> PtiBeR {
        PtiBeR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Background (BK) packet traffic priority level"]
    #[inline(always)]
    pub fn pti_bk(&self) -> PtiBkR {
        PtiBkR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Beacon (BCN) packet traffic priority level"]
    #[inline(always)]
    pub fn pti_bcn(&self) -> PtiBcnR {
        PtiBcnR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ACK packet traffic priority level"]
    #[inline(always)]
    pub fn pti_ack(&mut self) -> PtiAckW<'_, CoexPtiSpec> {
        PtiAckW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Control packet traffic priority level"]
    #[inline(always)]
    pub fn pti_cntrl(&mut self) -> PtiCntrlW<'_, CoexPtiSpec> {
        PtiCntrlW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Management packet traffic priority level"]
    #[inline(always)]
    pub fn pti_mgt(&mut self) -> PtiMgtW<'_, CoexPtiSpec> {
        PtiMgtW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Voice (VO) packet traffic priority level"]
    #[inline(always)]
    pub fn pti_vo(&mut self) -> PtiVoW<'_, CoexPtiSpec> {
        PtiVoW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Video (VI) packet traffic priority level"]
    #[inline(always)]
    pub fn pti_vi(&mut self) -> PtiViW<'_, CoexPtiSpec> {
        PtiViW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Best Effort (BE) packet traffic priority level"]
    #[inline(always)]
    pub fn pti_be(&mut self) -> PtiBeW<'_, CoexPtiSpec> {
        PtiBeW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Background (BK) packet traffic priority level"]
    #[inline(always)]
    pub fn pti_bk(&mut self) -> PtiBkW<'_, CoexPtiSpec> {
        PtiBkW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Beacon (BCN) packet traffic priority level"]
    #[inline(always)]
    pub fn pti_bcn(&mut self) -> PtiBcnW<'_, CoexPtiSpec> {
        PtiBcnW::new(self, 28)
    }
}
#[doc = "Coexistence Packet Traffic Interface Priority Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`coex_pti::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`coex_pti::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoexPtiSpec;
impl crate::RegisterSpec for CoexPtiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`coex_pti::R`](R) reader structure"]
impl crate::Readable for CoexPtiSpec {}
#[doc = "`write(|w| ..)` method takes [`coex_pti::W`](W) writer structure"]
impl crate::Writable for CoexPtiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets coex_pti to value 0"]
impl crate::Resettable for CoexPtiSpec {}
