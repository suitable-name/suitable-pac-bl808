#[doc = "Register `SE_GMAC_0_LCA` reader"]
pub type R = crate::R<SeGmac0LcaSpec>;
#[doc = "Register `SE_GMAC_0_LCA` writer"]
pub type W = crate::W<SeGmac0LcaSpec>;
#[doc = "Field `se_gmac_0_lca` reader - "]
pub type SeGmac0LcaR = crate::FieldReader<u32>;
#[doc = "Field `se_gmac_0_lca` writer - "]
pub type SeGmac0LcaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_gmac_0_lca(&self) -> SeGmac0LcaR {
        SeGmac0LcaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_gmac_0_lca(&mut self) -> SeGmac0LcaW<'_, SeGmac0LcaSpec> {
        SeGmac0LcaW::new(self, 0)
    }
}
#[doc = "GMAC Link List Address\n\nYou can [`read`](crate::Reg::read) this register and get [`se_gmac_0_lca::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_gmac_0_lca::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeGmac0LcaSpec;
impl crate::RegisterSpec for SeGmac0LcaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_gmac_0_lca::R`](R) reader structure"]
impl crate::Readable for SeGmac0LcaSpec {}
#[doc = "`write(|w| ..)` method takes [`se_gmac_0_lca::W`](W) writer structure"]
impl crate::Writable for SeGmac0LcaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_GMAC_0_LCA to value 0"]
impl crate::Resettable for SeGmac0LcaSpec {}
