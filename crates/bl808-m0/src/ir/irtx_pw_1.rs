#[doc = "Register `IRTX_PW_1` reader"]
pub type R = crate::R<IrtxPw1Spec>;
#[doc = "Register `IRTX_PW_1` writer"]
pub type W = crate::W<IrtxPw1Spec>;
#[doc = "Field `cr_irtx_head_ph0_w` reader - Head phase 0 width"]
pub type CrIrtxHeadPh0WR = crate::FieldReader;
#[doc = "Field `cr_irtx_head_ph0_w` writer - Head phase 0 width"]
pub type CrIrtxHeadPh0WW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_irtx_head_ph1_w` reader - Head phase 1 width"]
pub type CrIrtxHeadPh1WR = crate::FieldReader;
#[doc = "Field `cr_irtx_head_ph1_w` writer - Head phase 1 width"]
pub type CrIrtxHeadPh1WW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_irtx_tail_ph0_w` reader - Tail phase 0 width"]
pub type CrIrtxTailPh0WR = crate::FieldReader;
#[doc = "Field `cr_irtx_tail_ph0_w` writer - Tail phase 0 width"]
pub type CrIrtxTailPh0WW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_irtx_tail_ph1_w` reader - Tail phase 1 width"]
pub type CrIrtxTailPh1WR = crate::FieldReader;
#[doc = "Field `cr_irtx_tail_ph1_w` writer - Tail phase 1 width"]
pub type CrIrtxTailPh1WW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Head phase 0 width"]
    #[inline(always)]
    pub fn cr_irtx_head_ph0_w(&self) -> CrIrtxHeadPh0WR {
        CrIrtxHeadPh0WR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Head phase 1 width"]
    #[inline(always)]
    pub fn cr_irtx_head_ph1_w(&self) -> CrIrtxHeadPh1WR {
        CrIrtxHeadPh1WR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Tail phase 0 width"]
    #[inline(always)]
    pub fn cr_irtx_tail_ph0_w(&self) -> CrIrtxTailPh0WR {
        CrIrtxTailPh0WR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Tail phase 1 width"]
    #[inline(always)]
    pub fn cr_irtx_tail_ph1_w(&self) -> CrIrtxTailPh1WR {
        CrIrtxTailPh1WR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Head phase 0 width"]
    #[inline(always)]
    pub fn cr_irtx_head_ph0_w(&mut self) -> CrIrtxHeadPh0WW<'_, IrtxPw1Spec> {
        CrIrtxHeadPh0WW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Head phase 1 width"]
    #[inline(always)]
    pub fn cr_irtx_head_ph1_w(&mut self) -> CrIrtxHeadPh1WW<'_, IrtxPw1Spec> {
        CrIrtxHeadPh1WW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Tail phase 0 width"]
    #[inline(always)]
    pub fn cr_irtx_tail_ph0_w(&mut self) -> CrIrtxTailPh0WW<'_, IrtxPw1Spec> {
        CrIrtxTailPh0WW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Tail phase 1 width"]
    #[inline(always)]
    pub fn cr_irtx_tail_ph1_w(&mut self) -> CrIrtxTailPh1WW<'_, IrtxPw1Spec> {
        CrIrtxTailPh1WW::new(self, 24)
    }
}
#[doc = "IRTX Head/Tail Pulse Width Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_pw_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_pw_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrtxPw1Spec;
impl crate::RegisterSpec for IrtxPw1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irtx_pw_1::R`](R) reader structure"]
impl crate::Readable for IrtxPw1Spec {}
#[doc = "`write(|w| ..)` method takes [`irtx_pw_1::W`](W) writer structure"]
impl crate::Writable for IrtxPw1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRTX_PW_1 to value 0x070f"]
impl crate::Resettable for IrtxPw1Spec {
    const RESET_VALUE: u32 = 0x070f;
}
