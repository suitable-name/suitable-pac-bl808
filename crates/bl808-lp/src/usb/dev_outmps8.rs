#[doc = "Register `DEV_OUTMPS8` reader"]
pub type R = crate::R<DevOutmps8Spec>;
#[doc = "Register `DEV_OUTMPS8` writer"]
pub type W = crate::W<DevOutmps8Spec>;
#[doc = "Field `MAXPS_OEP8` reader - "]
pub type MaxpsOep8R = crate::FieldReader<u16>;
#[doc = "Field `MAXPS_OEP8` writer - "]
pub type MaxpsOep8W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `STL_OEP8` reader - "]
pub type StlOep8R = crate::BitReader;
#[doc = "Field `STL_OEP8` writer - "]
pub type StlOep8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTG_OEP8` reader - "]
pub type RstgOep8R = crate::BitReader;
#[doc = "Field `RSTG_OEP8` writer - "]
pub type RstgOep8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_oep8(&self) -> MaxpsOep8R {
        MaxpsOep8R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stl_oep8(&self) -> StlOep8R {
        StlOep8R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rstg_oep8(&self) -> RstgOep8R {
        RstgOep8R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_oep8(&mut self) -> MaxpsOep8W<'_, DevOutmps8Spec> {
        MaxpsOep8W::new(self, 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stl_oep8(&mut self) -> StlOep8W<'_, DevOutmps8Spec> {
        StlOep8W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rstg_oep8(&mut self) -> RstgOep8W<'_, DevOutmps8Spec> {
        RstgOep8W::new(self, 12)
    }
}
#[doc = "Device OUT Endpoint 8 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_outmps8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_outmps8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevOutmps8Spec;
impl crate::RegisterSpec for DevOutmps8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_outmps8::R`](R) reader structure"]
impl crate::Readable for DevOutmps8Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_outmps8::W`](W) writer structure"]
impl crate::Writable for DevOutmps8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_OUTMPS8 to value 0"]
impl crate::Resettable for DevOutmps8Spec {}
