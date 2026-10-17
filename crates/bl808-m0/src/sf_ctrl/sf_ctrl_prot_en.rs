#[doc = "Register `SF_CTRL_PROT_EN` reader"]
pub type R = crate::R<SfCtrlProtEnSpec>;
#[doc = "Register `SF_CTRL_PROT_EN` writer"]
pub type W = crate::W<SfCtrlProtEnSpec>;
#[doc = "Field `sf_ctrl_id0_en` reader - "]
pub type SfCtrlId0EnR = crate::BitReader;
#[doc = "Field `sf_ctrl_id0_en` writer - "]
pub type SfCtrlId0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_ctrl_id1_en` reader - "]
pub type SfCtrlId1EnR = crate::BitReader;
#[doc = "Field `sf_ctrl_id1_en` writer - "]
pub type SfCtrlId1EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sf_ctrl_id0_en(&self) -> SfCtrlId0EnR {
        SfCtrlId0EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_ctrl_id1_en(&self) -> SfCtrlId1EnR {
        SfCtrlId1EnR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sf_ctrl_id0_en(&mut self) -> SfCtrlId0EnW<'_, SfCtrlProtEnSpec> {
        SfCtrlId0EnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_ctrl_id1_en(&mut self) -> SfCtrlId1EnW<'_, SfCtrlProtEnSpec> {
        SfCtrlId1EnW::new(self, 2)
    }
}
#[doc = "TrustZone Protection Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ctrl_prot_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_ctrl_prot_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfCtrlProtEnSpec;
impl crate::RegisterSpec for SfCtrlProtEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_ctrl_prot_en::R`](R) reader structure"]
impl crate::Readable for SfCtrlProtEnSpec {}
#[doc = "`write(|w| ..)` method takes [`sf_ctrl_prot_en::W`](W) writer structure"]
impl crate::Writable for SfCtrlProtEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_CTRL_PROT_EN to value 0"]
impl crate::Resettable for SfCtrlProtEnSpec {}
