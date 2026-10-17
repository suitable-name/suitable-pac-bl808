#[doc = "Register `BLAI_LMTR_WR` reader"]
pub type R = crate::R<BlaiLmtrWrSpec>;
#[doc = "Register `BLAI_LMTR_WR` writer"]
pub type W = crate::W<BlaiLmtrWrSpec>;
#[doc = "Field `REG_BLAI_WCMD_CNT` reader - Number of write commands allowed per period"]
pub type RegBlaiWcmdCntR = crate::FieldReader<u16>;
#[doc = "Field `REG_BLAI_WCMD_CNT` writer - Number of write commands allowed per period"]
pub type RegBlaiWcmdCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REG_BLAI_WCMD_MODE` reader - Write command limit mode enable"]
pub type RegBlaiWcmdModeR = crate::BitReader;
#[doc = "Field `REG_BLAI_WCMD_MODE` writer - Write command limit mode enable"]
pub type RegBlaiWcmdModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Number of write commands allowed per period"]
    #[inline(always)]
    pub fn reg_blai_wcmd_cnt(&self) -> RegBlaiWcmdCntR {
        RegBlaiWcmdCntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Write command limit mode enable"]
    #[inline(always)]
    pub fn reg_blai_wcmd_mode(&self) -> RegBlaiWcmdModeR {
        RegBlaiWcmdModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of write commands allowed per period"]
    #[inline(always)]
    pub fn reg_blai_wcmd_cnt(&mut self) -> RegBlaiWcmdCntW<'_, BlaiLmtrWrSpec> {
        RegBlaiWcmdCntW::new(self, 0)
    }
    #[doc = "Bit 31 - Write command limit mode enable"]
    #[inline(always)]
    pub fn reg_blai_wcmd_mode(&mut self) -> RegBlaiWcmdModeW<'_, BlaiLmtrWrSpec> {
        RegBlaiWcmdModeW::new(self, 31)
    }
}
#[doc = "NPU (BLAI) Write Command Limiter\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_lmtr_wr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_lmtr_wr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiLmtrWrSpec;
impl crate::RegisterSpec for BlaiLmtrWrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_lmtr_wr::R`](R) reader structure"]
impl crate::Readable for BlaiLmtrWrSpec {}
#[doc = "`write(|w| ..)` method takes [`blai_lmtr_wr::W`](W) writer structure"]
impl crate::Writable for BlaiLmtrWrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_LMTR_WR to value 0"]
impl crate::Resettable for BlaiLmtrWrSpec {}
