#[doc = "Register `DEV_OUTMPS6` reader"]
pub type R = crate::R<DevOutmps6Spec>;
#[doc = "Register `DEV_OUTMPS6` writer"]
pub type W = crate::W<DevOutmps6Spec>;
#[doc = "Field `MAXPS_OEP6` reader - "]
pub type MaxpsOep6R = crate::FieldReader<u16>;
#[doc = "Field `MAXPS_OEP6` writer - "]
pub type MaxpsOep6W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `STL_OEP6` reader - "]
pub type StlOep6R = crate::BitReader;
#[doc = "Field `STL_OEP6` writer - "]
pub type StlOep6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTG_OEP6` reader - "]
pub type RstgOep6R = crate::BitReader;
#[doc = "Field `RSTG_OEP6` writer - "]
pub type RstgOep6W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_oep6(&self) -> MaxpsOep6R {
        MaxpsOep6R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stl_oep6(&self) -> StlOep6R {
        StlOep6R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rstg_oep6(&self) -> RstgOep6R {
        RstgOep6R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_oep6(&mut self) -> MaxpsOep6W<'_, DevOutmps6Spec> {
        MaxpsOep6W::new(self, 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stl_oep6(&mut self) -> StlOep6W<'_, DevOutmps6Spec> {
        StlOep6W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rstg_oep6(&mut self) -> RstgOep6W<'_, DevOutmps6Spec> {
        RstgOep6W::new(self, 12)
    }
}
#[doc = "Device OUT Endpoint 6 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_outmps6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_outmps6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevOutmps6Spec;
impl crate::RegisterSpec for DevOutmps6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_outmps6::R`](R) reader structure"]
impl crate::Readable for DevOutmps6Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_outmps6::W`](W) writer structure"]
impl crate::Writable for DevOutmps6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_OUTMPS6 to value 0"]
impl crate::Resettable for DevOutmps6Spec {}
