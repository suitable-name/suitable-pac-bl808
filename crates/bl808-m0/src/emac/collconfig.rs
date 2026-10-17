#[doc = "Register `COLLCONFIG` reader"]
pub type R = crate::R<CollconfigSpec>;
#[doc = "Register `COLLCONFIG` writer"]
pub type W = crate::W<CollconfigSpec>;
#[doc = "Field `COLLVALID` reader - "]
pub type CollvalidR = crate::FieldReader;
#[doc = "Field `COLLVALID` writer - "]
pub type CollvalidW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MAXRET` reader - "]
pub type MaxretR = crate::FieldReader;
#[doc = "Field `MAXRET` writer - "]
pub type MaxretW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn collvalid(&self) -> CollvalidR {
        CollvalidR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn maxret(&self) -> MaxretR {
        MaxretR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn collvalid(&mut self) -> CollvalidW<'_, CollconfigSpec> {
        CollvalidW::new(self, 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn maxret(&mut self) -> MaxretW<'_, CollconfigSpec> {
        MaxretW::new(self, 16)
    }
}
#[doc = "Collision Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`collconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`collconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CollconfigSpec;
impl crate::RegisterSpec for CollconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`collconfig::R`](R) reader structure"]
impl crate::Readable for CollconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`collconfig::W`](W) writer structure"]
impl crate::Writable for CollconfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COLLCONFIG to value 0x000f_003f"]
impl crate::Resettable for CollconfigSpec {
    const RESET_VALUE: u32 = 0x000f_003f;
}
