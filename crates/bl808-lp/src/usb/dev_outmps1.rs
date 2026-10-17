#[doc = "Register `DEV_OUTMPS1` reader"]
pub type R = crate::R<DevOutmps1Spec>;
#[doc = "Register `DEV_OUTMPS1` writer"]
pub type W = crate::W<DevOutmps1Spec>;
#[doc = "Field `MAXPS_OEP1` reader - "]
pub type MaxpsOep1R = crate::FieldReader<u16>;
#[doc = "Field `MAXPS_OEP1` writer - "]
pub type MaxpsOep1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `STL_OEP1` reader - Stall Endpoint 1"]
pub type StlOep1R = crate::BitReader;
#[doc = "Field `STL_OEP1` writer - Stall Endpoint 1"]
pub type StlOep1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTG_OEP1` reader - Reset Toggle for Endpoint 1"]
pub type RstgOep1R = crate::BitReader;
#[doc = "Field `RSTG_OEP1` writer - Reset Toggle for Endpoint 1"]
pub type RstgOep1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_oep1(&self) -> MaxpsOep1R {
        MaxpsOep1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Stall Endpoint 1"]
    #[inline(always)]
    pub fn stl_oep1(&self) -> StlOep1R {
        StlOep1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reset Toggle for Endpoint 1"]
    #[inline(always)]
    pub fn rstg_oep1(&self) -> RstgOep1R {
        RstgOep1R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_oep1(&mut self) -> MaxpsOep1W<'_, DevOutmps1Spec> {
        MaxpsOep1W::new(self, 0)
    }
    #[doc = "Bit 11 - Stall Endpoint 1"]
    #[inline(always)]
    pub fn stl_oep1(&mut self) -> StlOep1W<'_, DevOutmps1Spec> {
        StlOep1W::new(self, 11)
    }
    #[doc = "Bit 12 - Reset Toggle for Endpoint 1"]
    #[inline(always)]
    pub fn rstg_oep1(&mut self) -> RstgOep1W<'_, DevOutmps1Spec> {
        RstgOep1W::new(self, 12)
    }
}
#[doc = "Device OUT Endpoint 1 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_outmps1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_outmps1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevOutmps1Spec;
impl crate::RegisterSpec for DevOutmps1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_outmps1::R`](R) reader structure"]
impl crate::Readable for DevOutmps1Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_outmps1::W`](W) writer structure"]
impl crate::Writable for DevOutmps1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_OUTMPS1 to value 0"]
impl crate::Resettable for DevOutmps1Spec {}
