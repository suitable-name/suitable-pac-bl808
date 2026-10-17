#[doc = "Register `SE_CTRL_RESERVED_2` reader"]
pub type R = crate::R<SeCtrlReserved2Spec>;
#[doc = "Field `se_ctrl_reserved_2` reader - Hardware Version/Revision ID code."]
pub type SeCtrlReserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Hardware Version/Revision ID code."]
    #[inline(always)]
    pub fn se_ctrl_reserved_2(&self) -> SeCtrlReserved2R {
        SeCtrlReserved2R::new(self.bits)
    }
}
#[doc = "Security engine hardware revision identification. Returns the silicon-specific version code for the encryption accelerators.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_ctrl_reserved_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeCtrlReserved2Spec;
impl crate::RegisterSpec for SeCtrlReserved2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_ctrl_reserved_2::R`](R) reader structure"]
impl crate::Readable for SeCtrlReserved2Spec {}
#[doc = "`reset()` method sets SE_CTRL_RESERVED_2 to value 0"]
impl crate::Resettable for SeCtrlReserved2Spec {}
