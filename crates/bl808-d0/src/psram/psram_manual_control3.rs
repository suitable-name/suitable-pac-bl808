#[doc = "Register `PSRAM_MANUAL_CONTROL3` reader"]
pub type R = crate::R<PsramManualControl3Spec>;
#[doc = "Register `PSRAM_MANUAL_CONTROL3` writer"]
pub type W = crate::W<PsramManualControl3Spec>;
#[doc = "Field `reg_adq_rel_val` reader - ADQ relative value"]
pub type RegAdqRelValR = crate::FieldReader;
#[doc = "Field `reg_adq_rel_val` writer - ADQ relative value"]
pub type RegAdqRelValW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `reg_wrap2incr_en` reader - Wrap to increment enable"]
pub type RegWrap2incrEnR = crate::BitReader;
#[doc = "Field `reg_wrap2incr_en` writer - Wrap to increment enable"]
pub type RegWrap2incrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - ADQ relative value"]
    #[inline(always)]
    pub fn reg_adq_rel_val(&self) -> RegAdqRelValR {
        RegAdqRelValR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Wrap to increment enable"]
    #[inline(always)]
    pub fn reg_wrap2incr_en(&self) -> RegWrap2incrEnR {
        RegWrap2incrEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - ADQ relative value"]
    #[inline(always)]
    pub fn reg_adq_rel_val(&mut self) -> RegAdqRelValW<'_, PsramManualControl3Spec> {
        RegAdqRelValW::new(self, 0)
    }
    #[doc = "Bit 8 - Wrap to increment enable"]
    #[inline(always)]
    pub fn reg_wrap2incr_en(&mut self) -> RegWrap2incrEnW<'_, PsramManualControl3Spec> {
        RegWrap2incrEnW::new(self, 8)
    }
}
#[doc = "PSRAM Manual Control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_manual_control3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_manual_control3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramManualControl3Spec;
impl crate::RegisterSpec for PsramManualControl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_manual_control3::R`](R) reader structure"]
impl crate::Readable for PsramManualControl3Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_manual_control3::W`](W) writer structure"]
impl crate::Writable for PsramManualControl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_MANUAL_CONTROL3 to value 0x0120"]
impl crate::Resettable for PsramManualControl3Spec {
    const RESET_VALUE: u32 = 0x0120;
}
