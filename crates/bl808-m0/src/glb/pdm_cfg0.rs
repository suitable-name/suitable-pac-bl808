#[doc = "Register `PDM_CFG0` reader"]
pub type R = crate::R<PdmCfg0Spec>;
#[doc = "Register `PDM_CFG0` writer"]
pub type W = crate::W<PdmCfg0Spec>;
#[doc = "Field `reg_pdm_io_sel` reader - "]
pub type RegPdmIoSelR = crate::BitReader;
#[doc = "Field `reg_pdm_io_sel` writer - "]
pub type RegPdmIoSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_pdm_io_sel(&self) -> RegPdmIoSelR {
        RegPdmIoSelR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_pdm_io_sel(&mut self) -> RegPdmIoSelW<'_, PdmCfg0Spec> {
        RegPdmIoSelW::new(self, 0)
    }
}
#[doc = "PDM IO Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pdm_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdm_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdmCfg0Spec;
impl crate::RegisterSpec for PdmCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdm_cfg0::R`](R) reader structure"]
impl crate::Readable for PdmCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`pdm_cfg0::W`](W) writer structure"]
impl crate::Writable for PdmCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDM_CFG0 to value 0"]
impl crate::Resettable for PdmCfg0Spec {}
