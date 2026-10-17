#[doc = "Register `SE_TRNG_0_CTRL_PROT` reader"]
pub type R = crate::R<SeTrng0CtrlProtSpec>;
#[doc = "Register `SE_TRNG_0_CTRL_PROT` writer"]
pub type W = crate::W<SeTrng0CtrlProtSpec>;
#[doc = "Field `se_trng_id0_en` reader - "]
pub type SeTrngId0EnR = crate::BitReader;
#[doc = "Field `se_trng_id0_en` writer - "]
pub type SeTrngId0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_trng_id1_en` reader - "]
pub type SeTrngId1EnR = crate::BitReader;
#[doc = "Field `se_trng_id1_en` writer - "]
pub type SeTrngId1EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_trng_id0_en(&self) -> SeTrngId0EnR {
        SeTrngId0EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_trng_id1_en(&self) -> SeTrngId1EnR {
        SeTrngId1EnR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_trng_id0_en(&mut self) -> SeTrngId0EnW<'_, SeTrng0CtrlProtSpec> {
        SeTrngId0EnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_trng_id1_en(&mut self) -> SeTrngId1EnW<'_, SeTrng0CtrlProtSpec> {
        SeTrngId1EnW::new(self, 2)
    }
}
#[doc = "TRNG Engine Protection\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_ctrl_prot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_trng_0_ctrl_prot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeTrng0CtrlProtSpec;
impl crate::RegisterSpec for SeTrng0CtrlProtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_trng_0_ctrl_prot::R`](R) reader structure"]
impl crate::Readable for SeTrng0CtrlProtSpec {}
#[doc = "`write(|w| ..)` method takes [`se_trng_0_ctrl_prot::W`](W) writer structure"]
impl crate::Writable for SeTrng0CtrlProtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_TRNG_0_CTRL_PROT to value 0x06"]
impl crate::Resettable for SeTrng0CtrlProtSpec {
    const RESET_VALUE: u32 = 0x06;
}
