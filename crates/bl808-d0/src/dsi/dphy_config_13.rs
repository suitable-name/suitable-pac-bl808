#[doc = "Register `DPHY_CONFIG_13` reader"]
pub type R = crate::R<DphyConfig13Spec>;
#[doc = "Field `reg_rd_dig_test_bus` reader - "]
pub type RegRdDigTestBusR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn reg_rd_dig_test_bus(&self) -> RegRdDigTestBusR {
        RegRdDigTestBusR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "DPHY Digital Test Bus Readout\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_13::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DphyConfig13Spec;
impl crate::RegisterSpec for DphyConfig13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dphy_config_13::R`](R) reader structure"]
impl crate::Readable for DphyConfig13Spec {}
#[doc = "`reset()` method sets DPHY_CONFIG_13 to value 0"]
impl crate::Resettable for DphyConfig13Spec {}
