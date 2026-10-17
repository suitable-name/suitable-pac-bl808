#[doc = "Register `SE_CDET_0_CTRL_PROT` reader"]
pub type R = crate::R<SeCdet0CtrlProtSpec>;
#[doc = "Register `SE_CDET_0_CTRL_PROT` writer"]
pub type W = crate::W<SeCdet0CtrlProtSpec>;
#[doc = "Field `se_cdet_prot_en` reader - "]
pub type SeCdetProtEnR = crate::BitReader;
#[doc = "Field `se_cdet_prot_en` writer - "]
pub type SeCdetProtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_cdet_id0_en` reader - "]
pub type SeCdetId0EnR = crate::BitReader;
#[doc = "Field `se_cdet_id0_en` writer - "]
pub type SeCdetId0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_cdet_id1_en` reader - "]
pub type SeCdetId1EnR = crate::BitReader;
#[doc = "Field `se_cdet_id1_en` writer - "]
pub type SeCdetId1EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_cdet_prot_en(&self) -> SeCdetProtEnR {
        SeCdetProtEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_cdet_id0_en(&self) -> SeCdetId0EnR {
        SeCdetId0EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_cdet_id1_en(&self) -> SeCdetId1EnR {
        SeCdetId1EnR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_cdet_prot_en(&mut self) -> SeCdetProtEnW<'_, SeCdet0CtrlProtSpec> {
        SeCdetProtEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_cdet_id0_en(&mut self) -> SeCdetId0EnW<'_, SeCdet0CtrlProtSpec> {
        SeCdetId0EnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_cdet_id1_en(&mut self) -> SeCdetId1EnW<'_, SeCdet0CtrlProtSpec> {
        SeCdetId1EnW::new(self, 2)
    }
}
#[doc = "CDET Engine Protection\n\nYou can [`read`](crate::Reg::read) this register and get [`se_cdet_0_ctrl_prot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_cdet_0_ctrl_prot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeCdet0CtrlProtSpec;
impl crate::RegisterSpec for SeCdet0CtrlProtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_cdet_0_ctrl_prot::R`](R) reader structure"]
impl crate::Readable for SeCdet0CtrlProtSpec {}
#[doc = "`write(|w| ..)` method takes [`se_cdet_0_ctrl_prot::W`](W) writer structure"]
impl crate::Writable for SeCdet0CtrlProtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_CDET_0_CTRL_PROT to value 0x07"]
impl crate::Resettable for SeCdet0CtrlProtSpec {
    const RESET_VALUE: u32 = 0x07;
}
