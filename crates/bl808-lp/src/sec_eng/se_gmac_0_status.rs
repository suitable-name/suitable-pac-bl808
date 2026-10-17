#[doc = "Register `SE_GMAC_0_STATUS` reader"]
pub type R = crate::R<SeGmac0StatusSpec>;
#[doc = "Field `se_gmac_0_status` reader - "]
pub type SeGmac0StatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_gmac_0_status(&self) -> SeGmac0StatusR {
        SeGmac0StatusR::new(self.bits)
    }
}
#[doc = "GMAC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`se_gmac_0_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeGmac0StatusSpec;
impl crate::RegisterSpec for SeGmac0StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_gmac_0_status::R`](R) reader structure"]
impl crate::Readable for SeGmac0StatusSpec {}
#[doc = "`reset()` method sets SE_GMAC_0_STATUS to value 0xf100_0000"]
impl crate::Resettable for SeGmac0StatusSpec {
    const RESET_VALUE: u32 = 0xf100_0000;
}
