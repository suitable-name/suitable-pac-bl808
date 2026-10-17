#[doc = "Register `SE_AES_0_CTRL_PROT` reader"]
pub type R = crate::R<SeAes0CtrlProtSpec>;
#[doc = "Register `SE_AES_0_CTRL_PROT` writer"]
pub type W = crate::W<SeAes0CtrlProtSpec>;
#[doc = "Field `se_aes_id0_en` reader - "]
pub type SeAesId0EnR = crate::BitReader;
#[doc = "Field `se_aes_id0_en` writer - "]
pub type SeAesId0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_aes_id1_en` reader - "]
pub type SeAesId1EnR = crate::BitReader;
#[doc = "Field `se_aes_id1_en` writer - "]
pub type SeAesId1EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_aes_id0_en(&self) -> SeAesId0EnR {
        SeAesId0EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_aes_id1_en(&self) -> SeAesId1EnR {
        SeAesId1EnR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_aes_id0_en(&mut self) -> SeAesId0EnW<'_, SeAes0CtrlProtSpec> {
        SeAesId0EnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_aes_id1_en(&mut self) -> SeAesId1EnW<'_, SeAes0CtrlProtSpec> {
        SeAesId1EnW::new(self, 2)
    }
}
#[doc = "AES Engine Protection\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_ctrl_prot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_ctrl_prot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeAes0CtrlProtSpec;
impl crate::RegisterSpec for SeAes0CtrlProtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_aes_0_ctrl_prot::R`](R) reader structure"]
impl crate::Readable for SeAes0CtrlProtSpec {}
#[doc = "`write(|w| ..)` method takes [`se_aes_0_ctrl_prot::W`](W) writer structure"]
impl crate::Writable for SeAes0CtrlProtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_AES_0_CTRL_PROT to value 0x06"]
impl crate::Resettable for SeAes0CtrlProtSpec {
    const RESET_VALUE: u32 = 0x06;
}
