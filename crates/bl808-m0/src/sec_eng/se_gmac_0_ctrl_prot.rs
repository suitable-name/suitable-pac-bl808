#[doc = "Register `SE_GMAC_0_CTRL_PROT` reader"]
pub type R = crate::R<SeGmac0CtrlProtSpec>;
#[doc = "Register `SE_GMAC_0_CTRL_PROT` writer"]
pub type W = crate::W<SeGmac0CtrlProtSpec>;
#[doc = "Field `se_gmac_id0_en` reader - "]
pub type SeGmacId0EnR = crate::BitReader;
#[doc = "Field `se_gmac_id0_en` writer - "]
pub type SeGmacId0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_gmac_id1_en` reader - "]
pub type SeGmacId1EnR = crate::BitReader;
#[doc = "Field `se_gmac_id1_en` writer - "]
pub type SeGmacId1EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_gmac_id0_en(&self) -> SeGmacId0EnR {
        SeGmacId0EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_gmac_id1_en(&self) -> SeGmacId1EnR {
        SeGmacId1EnR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_gmac_id0_en(&mut self) -> SeGmacId0EnW<'_, SeGmac0CtrlProtSpec> {
        SeGmacId0EnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_gmac_id1_en(&mut self) -> SeGmacId1EnW<'_, SeGmac0CtrlProtSpec> {
        SeGmacId1EnW::new(self, 2)
    }
}
#[doc = "GMAC Engine Protection\n\nYou can [`read`](crate::Reg::read) this register and get [`se_gmac_0_ctrl_prot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_gmac_0_ctrl_prot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeGmac0CtrlProtSpec;
impl crate::RegisterSpec for SeGmac0CtrlProtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_gmac_0_ctrl_prot::R`](R) reader structure"]
impl crate::Readable for SeGmac0CtrlProtSpec {}
#[doc = "`write(|w| ..)` method takes [`se_gmac_0_ctrl_prot::W`](W) writer structure"]
impl crate::Writable for SeGmac0CtrlProtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_GMAC_0_CTRL_PROT to value 0x06"]
impl crate::Resettable for SeGmac0CtrlProtSpec {
    const RESET_VALUE: u32 = 0x06;
}
