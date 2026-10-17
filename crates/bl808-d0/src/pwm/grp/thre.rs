#[doc = "Register `THRE%s` reader"]
pub type R = crate::R<ThreSpec>;
#[doc = "Register `THRE%s` writer"]
pub type W = crate::W<ThreSpec>;
#[doc = "Field `threL` reader - Threshold low value"]
pub type ThreLR = crate::FieldReader<u16>;
#[doc = "Field `threL` writer - Threshold low value"]
pub type ThreLW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `threH` reader - Threshold high value"]
pub type ThreHR = crate::FieldReader<u16>;
#[doc = "Field `threH` writer - Threshold high value"]
pub type ThreHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Threshold low value"]
    #[inline(always)]
    pub fn thre_l(&self) -> ThreLR {
        ThreLR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Threshold high value"]
    #[inline(always)]
    pub fn thre_h(&self) -> ThreHR {
        ThreHR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Threshold low value"]
    #[inline(always)]
    pub fn thre_l(&mut self) -> ThreLW<'_, ThreSpec> {
        ThreLW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Threshold high value"]
    #[inline(always)]
    pub fn thre_h(&mut self) -> ThreHW<'_, ThreSpec> {
        ThreHW::new(self, 16)
    }
}
#[doc = "PWM Channel %s Threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`thre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ThreSpec;
impl crate::RegisterSpec for ThreSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`thre::R`](R) reader structure"]
impl crate::Readable for ThreSpec {}
#[doc = "`write(|w| ..)` method takes [`thre::W`](W) writer structure"]
impl crate::Writable for ThreSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets THRE%s to value 0"]
impl crate::Resettable for ThreSpec {}
