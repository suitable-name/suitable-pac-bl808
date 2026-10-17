#[doc = "Register `IRTX_PULSE_WIDTH` reader"]
pub type R = crate::R<IrtxPulseWidthSpec>;
#[doc = "Register `IRTX_PULSE_WIDTH` writer"]
pub type W = crate::W<IrtxPulseWidthSpec>;
#[doc = "Field `cr_irtx_pw_unit` reader - IRTX pulse width unit cycles"]
pub type CrIrtxPwUnitR = crate::FieldReader<u16>;
#[doc = "Field `cr_irtx_pw_unit` writer - IRTX pulse width unit cycles"]
pub type CrIrtxPwUnitW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `cr_irtx_mod_ph0_w` reader - Modulation phase 0 width"]
pub type CrIrtxModPh0WR = crate::FieldReader;
#[doc = "Field `cr_irtx_mod_ph0_w` writer - Modulation phase 0 width"]
pub type CrIrtxModPh0WW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_irtx_mod_ph1_w` reader - Modulation phase 1 width"]
pub type CrIrtxModPh1WR = crate::FieldReader;
#[doc = "Field `cr_irtx_mod_ph1_w` writer - Modulation phase 1 width"]
pub type CrIrtxModPh1WW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:11 - IRTX pulse width unit cycles"]
    #[inline(always)]
    pub fn cr_irtx_pw_unit(&self) -> CrIrtxPwUnitR {
        CrIrtxPwUnitR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:23 - Modulation phase 0 width"]
    #[inline(always)]
    pub fn cr_irtx_mod_ph0_w(&self) -> CrIrtxModPh0WR {
        CrIrtxModPh0WR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Modulation phase 1 width"]
    #[inline(always)]
    pub fn cr_irtx_mod_ph1_w(&self) -> CrIrtxModPh1WR {
        CrIrtxModPh1WR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - IRTX pulse width unit cycles"]
    #[inline(always)]
    pub fn cr_irtx_pw_unit(&mut self) -> CrIrtxPwUnitW<'_, IrtxPulseWidthSpec> {
        CrIrtxPwUnitW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Modulation phase 0 width"]
    #[inline(always)]
    pub fn cr_irtx_mod_ph0_w(&mut self) -> CrIrtxModPh0WW<'_, IrtxPulseWidthSpec> {
        CrIrtxModPh0WW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Modulation phase 1 width"]
    #[inline(always)]
    pub fn cr_irtx_mod_ph1_w(&mut self) -> CrIrtxModPh1WW<'_, IrtxPulseWidthSpec> {
        CrIrtxModPh1WW::new(self, 24)
    }
}
#[doc = "IRTX Basic Pulse Width and Modulation Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_pulse_width::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_pulse_width::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrtxPulseWidthSpec;
impl crate::RegisterSpec for IrtxPulseWidthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irtx_pulse_width::R`](R) reader structure"]
impl crate::Readable for IrtxPulseWidthSpec {}
#[doc = "`write(|w| ..)` method takes [`irtx_pulse_width::W`](W) writer structure"]
impl crate::Writable for IrtxPulseWidthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRTX_PULSE_WIDTH to value 0x2211_0464"]
impl crate::Resettable for IrtxPulseWidthSpec {
    const RESET_VALUE: u32 = 0x2211_0464;
}
