#[doc = "Register `BUS_CFG0` reader"]
pub type R = crate::R<BusCfg0Spec>;
#[doc = "Register `BUS_CFG0` writer"]
pub type W = crate::W<BusCfg0Spec>;
#[doc = "Field `rg_apb2_pck_force` reader - "]
pub type RgApb2PckForceR = crate::FieldReader<u16>;
#[doc = "Field `rg_apb2_pck_force` writer - "]
pub type RgApb2PckForceW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rg_apb2_pck_force(&self) -> RgApb2PckForceR {
        RgApb2PckForceR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rg_apb2_pck_force(&mut self) -> RgApb2PckForceW<'_, BusCfg0Spec> {
        RgApb2PckForceW::new(self, 0)
    }
}
#[doc = "Bus Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusCfg0Spec;
impl crate::RegisterSpec for BusCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_cfg0::R`](R) reader structure"]
impl crate::Readable for BusCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`bus_cfg0::W`](W) writer structure"]
impl crate::Writable for BusCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUS_CFG0 to value 0"]
impl crate::Resettable for BusCfg0Spec {}
