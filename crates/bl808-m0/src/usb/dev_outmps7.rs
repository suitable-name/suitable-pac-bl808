#[doc = "Register `DEV_OUTMPS7` reader"]
pub type R = crate::R<DevOutmps7Spec>;
#[doc = "Register `DEV_OUTMPS7` writer"]
pub type W = crate::W<DevOutmps7Spec>;
#[doc = "Field `MAXPS_OEP7` reader - "]
pub type MaxpsOep7R = crate::FieldReader<u16>;
#[doc = "Field `MAXPS_OEP7` writer - "]
pub type MaxpsOep7W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `STL_OEP7` reader - "]
pub type StlOep7R = crate::BitReader;
#[doc = "Field `STL_OEP7` writer - "]
pub type StlOep7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTG_OEP7` reader - "]
pub type RstgOep7R = crate::BitReader;
#[doc = "Field `RSTG_OEP7` writer - "]
pub type RstgOep7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_oep7(&self) -> MaxpsOep7R {
        MaxpsOep7R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stl_oep7(&self) -> StlOep7R {
        StlOep7R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rstg_oep7(&self) -> RstgOep7R {
        RstgOep7R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_oep7(&mut self) -> MaxpsOep7W<'_, DevOutmps7Spec> {
        MaxpsOep7W::new(self, 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stl_oep7(&mut self) -> StlOep7W<'_, DevOutmps7Spec> {
        StlOep7W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rstg_oep7(&mut self) -> RstgOep7W<'_, DevOutmps7Spec> {
        RstgOep7W::new(self, 12)
    }
}
#[doc = "Device OUT Endpoint 7 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_outmps7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_outmps7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevOutmps7Spec;
impl crate::RegisterSpec for DevOutmps7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_outmps7::R`](R) reader structure"]
impl crate::Readable for DevOutmps7Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_outmps7::W`](W) writer structure"]
impl crate::Writable for DevOutmps7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_OUTMPS7 to value 0"]
impl crate::Resettable for DevOutmps7Spec {}
