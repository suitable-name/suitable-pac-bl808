#[doc = "Register `SE_SHA_0_CTRL_PROT` reader"]
pub type R = crate::R<SeSha0CtrlProtSpec>;
#[doc = "Register `SE_SHA_0_CTRL_PROT` writer"]
pub type W = crate::W<SeSha0CtrlProtSpec>;
#[doc = "Field `se_sha_id0_en` reader - "]
pub type SeShaId0EnR = crate::BitReader;
#[doc = "Field `se_sha_id0_en` writer - "]
pub type SeShaId0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_sha_id1_en` reader - "]
pub type SeShaId1EnR = crate::BitReader;
#[doc = "Field `se_sha_id1_en` writer - "]
pub type SeShaId1EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_sha_id0_en(&self) -> SeShaId0EnR {
        SeShaId0EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_sha_id1_en(&self) -> SeShaId1EnR {
        SeShaId1EnR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_sha_id0_en(&mut self) -> SeShaId0EnW<'_, SeSha0CtrlProtSpec> {
        SeShaId0EnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_sha_id1_en(&mut self) -> SeShaId1EnW<'_, SeSha0CtrlProtSpec> {
        SeShaId1EnW::new(self, 2)
    }
}
#[doc = "SHA Engine Protection\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_ctrl_prot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_sha_0_ctrl_prot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeSha0CtrlProtSpec;
impl crate::RegisterSpec for SeSha0CtrlProtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_sha_0_ctrl_prot::R`](R) reader structure"]
impl crate::Readable for SeSha0CtrlProtSpec {}
#[doc = "`write(|w| ..)` method takes [`se_sha_0_ctrl_prot::W`](W) writer structure"]
impl crate::Writable for SeSha0CtrlProtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_SHA_0_CTRL_PROT to value 0x06"]
impl crate::Resettable for SeSha0CtrlProtSpec {
    const RESET_VALUE: u32 = 0x06;
}
