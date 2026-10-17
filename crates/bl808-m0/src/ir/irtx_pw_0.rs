#[doc = "Register `IRTX_PW_0` reader"]
pub type R = crate::R<IrtxPw0Spec>;
#[doc = "Register `IRTX_PW_0` writer"]
pub type W = crate::W<IrtxPw0Spec>;
#[doc = "Field `cr_irtx_logic0_ph0_w` reader - Logic 0 phase 0 width"]
pub type CrIrtxLogic0Ph0WR = crate::FieldReader;
#[doc = "Field `cr_irtx_logic0_ph0_w` writer - Logic 0 phase 0 width"]
pub type CrIrtxLogic0Ph0WW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_irtx_logic0_ph1_w` reader - Logic 0 phase 1 width"]
pub type CrIrtxLogic0Ph1WR = crate::FieldReader;
#[doc = "Field `cr_irtx_logic0_ph1_w` writer - Logic 0 phase 1 width"]
pub type CrIrtxLogic0Ph1WW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_irtx_logic1_ph0_w` reader - Logic 1 phase 0 width"]
pub type CrIrtxLogic1Ph0WR = crate::FieldReader;
#[doc = "Field `cr_irtx_logic1_ph0_w` writer - Logic 1 phase 0 width"]
pub type CrIrtxLogic1Ph0WW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_irtx_logic1_ph1_w` reader - Logic 1 phase 1 width"]
pub type CrIrtxLogic1Ph1WR = crate::FieldReader;
#[doc = "Field `cr_irtx_logic1_ph1_w` writer - Logic 1 phase 1 width"]
pub type CrIrtxLogic1Ph1WW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Logic 0 phase 0 width"]
    #[inline(always)]
    pub fn cr_irtx_logic0_ph0_w(&self) -> CrIrtxLogic0Ph0WR {
        CrIrtxLogic0Ph0WR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Logic 0 phase 1 width"]
    #[inline(always)]
    pub fn cr_irtx_logic0_ph1_w(&self) -> CrIrtxLogic0Ph1WR {
        CrIrtxLogic0Ph1WR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Logic 1 phase 0 width"]
    #[inline(always)]
    pub fn cr_irtx_logic1_ph0_w(&self) -> CrIrtxLogic1Ph0WR {
        CrIrtxLogic1Ph0WR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Logic 1 phase 1 width"]
    #[inline(always)]
    pub fn cr_irtx_logic1_ph1_w(&self) -> CrIrtxLogic1Ph1WR {
        CrIrtxLogic1Ph1WR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Logic 0 phase 0 width"]
    #[inline(always)]
    pub fn cr_irtx_logic0_ph0_w(&mut self) -> CrIrtxLogic0Ph0WW<'_, IrtxPw0Spec> {
        CrIrtxLogic0Ph0WW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Logic 0 phase 1 width"]
    #[inline(always)]
    pub fn cr_irtx_logic0_ph1_w(&mut self) -> CrIrtxLogic0Ph1WW<'_, IrtxPw0Spec> {
        CrIrtxLogic0Ph1WW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Logic 1 phase 0 width"]
    #[inline(always)]
    pub fn cr_irtx_logic1_ph0_w(&mut self) -> CrIrtxLogic1Ph0WW<'_, IrtxPw0Spec> {
        CrIrtxLogic1Ph0WW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Logic 1 phase 1 width"]
    #[inline(always)]
    pub fn cr_irtx_logic1_ph1_w(&mut self) -> CrIrtxLogic1Ph1WW<'_, IrtxPw0Spec> {
        CrIrtxLogic1Ph1WW::new(self, 24)
    }
}
#[doc = "IRTX Logic 0/1 Pulse Width Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_pw_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_pw_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrtxPw0Spec;
impl crate::RegisterSpec for IrtxPw0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irtx_pw_0::R`](R) reader structure"]
impl crate::Readable for IrtxPw0Spec {}
#[doc = "`write(|w| ..)` method takes [`irtx_pw_0::W`](W) writer structure"]
impl crate::Writable for IrtxPw0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRTX_PW_0 to value 0x0200_0000"]
impl crate::Resettable for IrtxPw0Spec {
    const RESET_VALUE: u32 = 0x0200_0000;
}
