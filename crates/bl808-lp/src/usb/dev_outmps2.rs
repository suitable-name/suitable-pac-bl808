#[doc = "Register `DEV_OUTMPS2` reader"]
pub type R = crate::R<DevOutmps2Spec>;
#[doc = "Register `DEV_OUTMPS2` writer"]
pub type W = crate::W<DevOutmps2Spec>;
#[doc = "Field `MAXPS_OEP2` reader - "]
pub type MaxpsOep2R = crate::FieldReader<u16>;
#[doc = "Field `MAXPS_OEP2` writer - "]
pub type MaxpsOep2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `STL_OEP2` reader - "]
pub type StlOep2R = crate::BitReader;
#[doc = "Field `STL_OEP2` writer - "]
pub type StlOep2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTG_OEP2` reader - "]
pub type RstgOep2R = crate::BitReader;
#[doc = "Field `RSTG_OEP2` writer - "]
pub type RstgOep2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_oep2(&self) -> MaxpsOep2R {
        MaxpsOep2R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stl_oep2(&self) -> StlOep2R {
        StlOep2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rstg_oep2(&self) -> RstgOep2R {
        RstgOep2R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_oep2(&mut self) -> MaxpsOep2W<'_, DevOutmps2Spec> {
        MaxpsOep2W::new(self, 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stl_oep2(&mut self) -> StlOep2W<'_, DevOutmps2Spec> {
        StlOep2W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rstg_oep2(&mut self) -> RstgOep2W<'_, DevOutmps2Spec> {
        RstgOep2W::new(self, 12)
    }
}
#[doc = "Device OUT Endpoint 2 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_outmps2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_outmps2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevOutmps2Spec;
impl crate::RegisterSpec for DevOutmps2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_outmps2::R`](R) reader structure"]
impl crate::Readable for DevOutmps2Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_outmps2::W`](W) writer structure"]
impl crate::Writable for DevOutmps2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_OUTMPS2 to value 0"]
impl crate::Resettable for DevOutmps2Spec {}
