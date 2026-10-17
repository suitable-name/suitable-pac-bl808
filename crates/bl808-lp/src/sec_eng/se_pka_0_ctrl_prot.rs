#[doc = "Register `SE_PKA_0_CTRL_PROT` reader"]
pub type R = crate::R<SePka0CtrlProtSpec>;
#[doc = "Register `SE_PKA_0_CTRL_PROT` writer"]
pub type W = crate::W<SePka0CtrlProtSpec>;
#[doc = "Field `se_pka_id0_en` reader - "]
pub type SePkaId0EnR = crate::BitReader;
#[doc = "Field `se_pka_id0_en` writer - "]
pub type SePkaId0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_pka_id1_en` reader - "]
pub type SePkaId1EnR = crate::BitReader;
#[doc = "Field `se_pka_id1_en` writer - "]
pub type SePkaId1EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_pka_id0_en(&self) -> SePkaId0EnR {
        SePkaId0EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_pka_id1_en(&self) -> SePkaId1EnR {
        SePkaId1EnR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_pka_id0_en(&mut self) -> SePkaId0EnW<'_, SePka0CtrlProtSpec> {
        SePkaId0EnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_pka_id1_en(&mut self) -> SePkaId1EnW<'_, SePka0CtrlProtSpec> {
        SePkaId1EnW::new(self, 2)
    }
}
#[doc = "PKA Engine Protection\n\nYou can [`read`](crate::Reg::read) this register and get [`se_pka_0_ctrl_prot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_pka_0_ctrl_prot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SePka0CtrlProtSpec;
impl crate::RegisterSpec for SePka0CtrlProtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_pka_0_ctrl_prot::R`](R) reader structure"]
impl crate::Readable for SePka0CtrlProtSpec {}
#[doc = "`write(|w| ..)` method takes [`se_pka_0_ctrl_prot::W`](W) writer structure"]
impl crate::Writable for SePka0CtrlProtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_PKA_0_CTRL_PROT to value 0x06"]
impl crate::Resettable for SePka0CtrlProtSpec {
    const RESET_VALUE: u32 = 0x06;
}
