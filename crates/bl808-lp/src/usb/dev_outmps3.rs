#[doc = "Register `DEV_OUTMPS3` reader"]
pub type R = crate::R<DevOutmps3Spec>;
#[doc = "Register `DEV_OUTMPS3` writer"]
pub type W = crate::W<DevOutmps3Spec>;
#[doc = "Field `MAXPS_OEP3` reader - "]
pub type MaxpsOep3R = crate::FieldReader<u16>;
#[doc = "Field `MAXPS_OEP3` writer - "]
pub type MaxpsOep3W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `STL_OEP3` reader - "]
pub type StlOep3R = crate::BitReader;
#[doc = "Field `STL_OEP3` writer - "]
pub type StlOep3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTG_OEP3` reader - "]
pub type RstgOep3R = crate::BitReader;
#[doc = "Field `RSTG_OEP3` writer - "]
pub type RstgOep3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_oep3(&self) -> MaxpsOep3R {
        MaxpsOep3R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stl_oep3(&self) -> StlOep3R {
        StlOep3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rstg_oep3(&self) -> RstgOep3R {
        RstgOep3R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_oep3(&mut self) -> MaxpsOep3W<'_, DevOutmps3Spec> {
        MaxpsOep3W::new(self, 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stl_oep3(&mut self) -> StlOep3W<'_, DevOutmps3Spec> {
        StlOep3W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rstg_oep3(&mut self) -> RstgOep3W<'_, DevOutmps3Spec> {
        RstgOep3W::new(self, 12)
    }
}
#[doc = "Device OUT Endpoint 3 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_outmps3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_outmps3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevOutmps3Spec;
impl crate::RegisterSpec for DevOutmps3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_outmps3::R`](R) reader structure"]
impl crate::Readable for DevOutmps3Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_outmps3::W`](W) writer structure"]
impl crate::Writable for DevOutmps3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_OUTMPS3 to value 0"]
impl crate::Resettable for DevOutmps3Spec {}
