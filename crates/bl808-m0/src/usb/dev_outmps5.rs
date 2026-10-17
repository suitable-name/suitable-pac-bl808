#[doc = "Register `DEV_OUTMPS5` reader"]
pub type R = crate::R<DevOutmps5Spec>;
#[doc = "Register `DEV_OUTMPS5` writer"]
pub type W = crate::W<DevOutmps5Spec>;
#[doc = "Field `MAXPS_OEP5` reader - "]
pub type MaxpsOep5R = crate::FieldReader<u16>;
#[doc = "Field `MAXPS_OEP5` writer - "]
pub type MaxpsOep5W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `STL_OEP5` reader - "]
pub type StlOep5R = crate::BitReader;
#[doc = "Field `STL_OEP5` writer - "]
pub type StlOep5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTG_OEP5` reader - "]
pub type RstgOep5R = crate::BitReader;
#[doc = "Field `RSTG_OEP5` writer - "]
pub type RstgOep5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_oep5(&self) -> MaxpsOep5R {
        MaxpsOep5R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stl_oep5(&self) -> StlOep5R {
        StlOep5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rstg_oep5(&self) -> RstgOep5R {
        RstgOep5R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_oep5(&mut self) -> MaxpsOep5W<'_, DevOutmps5Spec> {
        MaxpsOep5W::new(self, 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stl_oep5(&mut self) -> StlOep5W<'_, DevOutmps5Spec> {
        StlOep5W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rstg_oep5(&mut self) -> RstgOep5W<'_, DevOutmps5Spec> {
        RstgOep5W::new(self, 12)
    }
}
#[doc = "Device OUT Endpoint 5 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_outmps5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_outmps5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevOutmps5Spec;
impl crate::RegisterSpec for DevOutmps5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_outmps5::R`](R) reader structure"]
impl crate::Readable for DevOutmps5Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_outmps5::W`](W) writer structure"]
impl crate::Writable for DevOutmps5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_OUTMPS5 to value 0"]
impl crate::Resettable for DevOutmps5Spec {}
