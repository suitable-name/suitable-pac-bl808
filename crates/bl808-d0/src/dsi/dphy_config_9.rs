#[doc = "Register `DPHY_CONFIG_9` reader"]
pub type R = crate::R<DphyConfig9Spec>;
#[doc = "Register `DPHY_CONFIG_9` writer"]
pub type W = crate::W<DphyConfig9Spec>;
#[doc = "Field `reg_ana_hs_data_out_byte` reader - "]
pub type RegAnaHsDataOutByteR = crate::FieldReader<u32>;
#[doc = "Field `reg_ana_hs_data_out_byte` writer - "]
pub type RegAnaHsDataOutByteW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_ana_hs_data_out_byte(&self) -> RegAnaHsDataOutByteR {
        RegAnaHsDataOutByteR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_ana_hs_data_out_byte(&mut self) -> RegAnaHsDataOutByteW<'_, DphyConfig9Spec> {
        RegAnaHsDataOutByteW::new(self, 0)
    }
}
#[doc = "DPHY HS Data Output\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DphyConfig9Spec;
impl crate::RegisterSpec for DphyConfig9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dphy_config_9::R`](R) reader structure"]
impl crate::Readable for DphyConfig9Spec {}
#[doc = "`write(|w| ..)` method takes [`dphy_config_9::W`](W) writer structure"]
impl crate::Writable for DphyConfig9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPHY_CONFIG_9 to value 0"]
impl crate::Resettable for DphyConfig9Spec {}
