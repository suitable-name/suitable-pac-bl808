#[doc = "Register `PSRAM_TIMEOUT_REG` reader"]
pub type R = crate::R<PsramTimeoutRegSpec>;
#[doc = "Register `PSRAM_TIMEOUT_REG` writer"]
pub type W = crate::W<PsramTimeoutRegSpec>;
#[doc = "Field `reg_timeout_en` reader - Timeout enable"]
pub type RegTimeoutEnR = crate::BitReader;
#[doc = "Field `reg_timeout_en` writer - Timeout enable"]
pub type RegTimeoutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_timeout_clr` writer - Timeout clear"]
pub type RegTimeoutClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `sts_timeout` reader - Timeout status"]
pub type StsTimeoutR = crate::BitReader;
#[doc = "Field `reg_timeout_cnt` reader - Timeout counter value"]
pub type RegTimeoutCntR = crate::FieldReader<u16>;
#[doc = "Field `reg_timeout_cnt` writer - Timeout counter value"]
pub type RegTimeoutCntW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - Timeout enable"]
    #[inline(always)]
    pub fn reg_timeout_en(&self) -> RegTimeoutEnR {
        RegTimeoutEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Timeout status"]
    #[inline(always)]
    pub fn sts_timeout(&self) -> StsTimeoutR {
        StsTimeoutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 16:27 - Timeout counter value"]
    #[inline(always)]
    pub fn reg_timeout_cnt(&self) -> RegTimeoutCntR {
        RegTimeoutCntR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Timeout enable"]
    #[inline(always)]
    pub fn reg_timeout_en(&mut self) -> RegTimeoutEnW<'_, PsramTimeoutRegSpec> {
        RegTimeoutEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Timeout clear"]
    #[inline(always)]
    pub fn reg_timeout_clr(&mut self) -> RegTimeoutClrW<'_, PsramTimeoutRegSpec> {
        RegTimeoutClrW::new(self, 1)
    }
    #[doc = "Bits 16:27 - Timeout counter value"]
    #[inline(always)]
    pub fn reg_timeout_cnt(&mut self) -> RegTimeoutCntW<'_, PsramTimeoutRegSpec> {
        RegTimeoutCntW::new(self, 16)
    }
}
#[doc = "Timeout Configuration and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_timeout_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_timeout_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramTimeoutRegSpec;
impl crate::RegisterSpec for PsramTimeoutRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_timeout_reg::R`](R) reader structure"]
impl crate::Readable for PsramTimeoutRegSpec {}
#[doc = "`write(|w| ..)` method takes [`psram_timeout_reg::W`](W) writer structure"]
impl crate::Writable for PsramTimeoutRegSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x02;
}
#[doc = "`reset()` method sets PSRAM_TIMEOUT_REG to value 0x0100_0000"]
impl crate::Resettable for PsramTimeoutRegSpec {
    const RESET_VALUE: u32 = 0x0100_0000;
}
