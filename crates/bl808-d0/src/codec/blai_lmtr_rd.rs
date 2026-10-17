#[doc = "Register `BLAI_LMTR_RD` reader"]
pub type R = crate::R<BlaiLmtrRdSpec>;
#[doc = "Register `BLAI_LMTR_RD` writer"]
pub type W = crate::W<BlaiLmtrRdSpec>;
#[doc = "Field `REG_BLAI_RCMD_CNT` reader - Number of read commands allowed per period"]
pub type RegBlaiRcmdCntR = crate::FieldReader<u16>;
#[doc = "Field `REG_BLAI_RCMD_CNT` writer - Number of read commands allowed per period"]
pub type RegBlaiRcmdCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REG_BLAI_RCMD_MODE` reader - Read command limit mode enable"]
pub type RegBlaiRcmdModeR = crate::BitReader;
#[doc = "Field `REG_BLAI_RCMD_MODE` writer - Read command limit mode enable"]
pub type RegBlaiRcmdModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Number of read commands allowed per period"]
    #[inline(always)]
    pub fn reg_blai_rcmd_cnt(&self) -> RegBlaiRcmdCntR {
        RegBlaiRcmdCntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Read command limit mode enable"]
    #[inline(always)]
    pub fn reg_blai_rcmd_mode(&self) -> RegBlaiRcmdModeR {
        RegBlaiRcmdModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of read commands allowed per period"]
    #[inline(always)]
    pub fn reg_blai_rcmd_cnt(&mut self) -> RegBlaiRcmdCntW<'_, BlaiLmtrRdSpec> {
        RegBlaiRcmdCntW::new(self, 0)
    }
    #[doc = "Bit 31 - Read command limit mode enable"]
    #[inline(always)]
    pub fn reg_blai_rcmd_mode(&mut self) -> RegBlaiRcmdModeW<'_, BlaiLmtrRdSpec> {
        RegBlaiRcmdModeW::new(self, 31)
    }
}
#[doc = "NPU (BLAI) Read Command Limiter\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_lmtr_rd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_lmtr_rd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiLmtrRdSpec;
impl crate::RegisterSpec for BlaiLmtrRdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_lmtr_rd::R`](R) reader structure"]
impl crate::Readable for BlaiLmtrRdSpec {}
#[doc = "`write(|w| ..)` method takes [`blai_lmtr_rd::W`](W) writer structure"]
impl crate::Writable for BlaiLmtrRdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_LMTR_RD to value 0"]
impl crate::Resettable for BlaiLmtrRdSpec {}
