#[doc = "Register `URX_BCR_INT_CFG` reader"]
pub type R = crate::R<UrxBcrIntCfgSpec>;
#[doc = "Register `URX_BCR_INT_CFG` writer"]
pub type W = crate::W<UrxBcrIntCfgSpec>;
#[doc = "Field `cr_urx_bcr_value` reader - "]
pub type CrUrxBcrValueR = crate::FieldReader<u16>;
#[doc = "Field `cr_urx_bcr_value` writer - "]
pub type CrUrxBcrValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `sts_urx_bcr_count` reader - "]
pub type StsUrxBcrCountR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_urx_bcr_value(&self) -> CrUrxBcrValueR {
        CrUrxBcrValueR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sts_urx_bcr_count(&self) -> StsUrxBcrCountR {
        StsUrxBcrCountR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_urx_bcr_value(&mut self) -> CrUrxBcrValueW<'_, UrxBcrIntCfgSpec> {
        CrUrxBcrValueW::new(self, 0)
    }
}
#[doc = "RX Byte Count Interrupt Config\n\nYou can [`read`](crate::Reg::read) this register and get [`urx_bcr_int_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`urx_bcr_int_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UrxBcrIntCfgSpec;
impl crate::RegisterSpec for UrxBcrIntCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`urx_bcr_int_cfg::R`](R) reader structure"]
impl crate::Readable for UrxBcrIntCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`urx_bcr_int_cfg::W`](W) writer structure"]
impl crate::Writable for UrxBcrIntCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets URX_BCR_INT_CFG to value 0xffff"]
impl crate::Resettable for UrxBcrIntCfgSpec {
    const RESET_VALUE: u32 = 0xffff;
}
