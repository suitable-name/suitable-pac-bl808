#[doc = "Register `dac_s1_misc` reader"]
pub type R = crate::R<DacS1MiscSpec>;
#[doc = "Register `dac_s1_misc` writer"]
pub type W = crate::W<DacS1MiscSpec>;
#[doc = "Field `dac_s1_ctrl_zcd_timeout` reader - "]
pub type DacS1CtrlZcdTimeoutR = crate::FieldReader;
#[doc = "Field `dac_s1_ctrl_zcd_timeout` writer - "]
pub type DacS1CtrlZcdTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn dac_s1_ctrl_zcd_timeout(&self) -> DacS1CtrlZcdTimeoutR {
        DacS1CtrlZcdTimeoutR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn dac_s1_ctrl_zcd_timeout(&mut self) -> DacS1CtrlZcdTimeoutW<'_, DacS1MiscSpec> {
        DacS1CtrlZcdTimeoutW::new(self, 28)
    }
}
#[doc = "DAC Sideband 1 misc: Second channel zero-crossing timeout\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_s1_misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_s1_misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacS1MiscSpec;
impl crate::RegisterSpec for DacS1MiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_s1_misc::R`](R) reader structure"]
impl crate::Readable for DacS1MiscSpec {}
#[doc = "`write(|w| ..)` method takes [`dac_s1_misc::W`](W) writer structure"]
impl crate::Writable for DacS1MiscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dac_s1_misc to value 0"]
impl crate::Resettable for DacS1MiscSpec {}
