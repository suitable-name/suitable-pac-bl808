#[doc = "Register `URX_ABR_PW_TOL` reader"]
pub type R = crate::R<UrxAbrPwTolSpec>;
#[doc = "Register `URX_ABR_PW_TOL` writer"]
pub type W = crate::W<UrxAbrPwTolSpec>;
#[doc = "Field `cr_urx_abr_pw_tol` reader - "]
pub type CrUrxAbrPwTolR = crate::FieldReader;
#[doc = "Field `cr_urx_abr_pw_tol` writer - "]
pub type CrUrxAbrPwTolW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_urx_abr_pw_tol(&self) -> CrUrxAbrPwTolR {
        CrUrxAbrPwTolR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_urx_abr_pw_tol(&mut self) -> CrUrxAbrPwTolW<'_, UrxAbrPwTolSpec> {
        CrUrxAbrPwTolW::new(self, 0)
    }
}
#[doc = "ABR Pulse Width Tolerance\n\nYou can [`read`](crate::Reg::read) this register and get [`urx_abr_pw_tol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`urx_abr_pw_tol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UrxAbrPwTolSpec;
impl crate::RegisterSpec for UrxAbrPwTolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`urx_abr_pw_tol::R`](R) reader structure"]
impl crate::Readable for UrxAbrPwTolSpec {}
#[doc = "`write(|w| ..)` method takes [`urx_abr_pw_tol::W`](W) writer structure"]
impl crate::Writable for UrxAbrPwTolSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets URX_ABR_PW_TOL to value 0"]
impl crate::Resettable for UrxAbrPwTolSpec {}
