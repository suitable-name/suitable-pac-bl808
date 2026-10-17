#[doc = "Register `PSRAM_ROUGH_DELAY_CTRL0` reader"]
pub type R = crate::R<PsramRoughDelayCtrl0Spec>;
#[doc = "Register `PSRAM_ROUGH_DELAY_CTRL0` writer"]
pub type W = crate::W<PsramRoughDelayCtrl0Spec>;
#[doc = "Field `reg_rough_sel_o_dqs_oen0` reader - Rough delay select for output DQS enable for slice 0"]
pub type RegRoughSelODqsOen0R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_dqs_oen0` writer - Rough delay select for output DQS enable for slice 0"]
pub type RegRoughSelODqsOen0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_o_ceb` reader - Rough delay select for output Chip Enable"]
pub type RegRoughSelOCebR = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_ceb` writer - Rough delay select for output Chip Enable"]
pub type RegRoughSelOCebW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_o_clk_n` reader - Rough delay select for output Clock Negative"]
pub type RegRoughSelOClkNR = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_clk_n` writer - Rough delay select for output Clock Negative"]
pub type RegRoughSelOClkNW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_o_clk` reader - Rough delay select for output Clock Positive"]
pub type RegRoughSelOClkR = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_clk` writer - Rough delay select for output Clock Positive"]
pub type RegRoughSelOClkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Rough delay select for output DQS enable for slice 0"]
    #[inline(always)]
    pub fn reg_rough_sel_o_dqs_oen0(&self) -> RegRoughSelODqsOen0R {
        RegRoughSelODqsOen0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Rough delay select for output Chip Enable"]
    #[inline(always)]
    pub fn reg_rough_sel_o_ceb(&self) -> RegRoughSelOCebR {
        RegRoughSelOCebR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Rough delay select for output Clock Negative"]
    #[inline(always)]
    pub fn reg_rough_sel_o_clk_n(&self) -> RegRoughSelOClkNR {
        RegRoughSelOClkNR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Rough delay select for output Clock Positive"]
    #[inline(always)]
    pub fn reg_rough_sel_o_clk(&self) -> RegRoughSelOClkR {
        RegRoughSelOClkR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Rough delay select for output DQS enable for slice 0"]
    #[inline(always)]
    pub fn reg_rough_sel_o_dqs_oen0(
        &mut self,
    ) -> RegRoughSelODqsOen0W<'_, PsramRoughDelayCtrl0Spec> {
        RegRoughSelODqsOen0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Rough delay select for output Chip Enable"]
    #[inline(always)]
    pub fn reg_rough_sel_o_ceb(&mut self) -> RegRoughSelOCebW<'_, PsramRoughDelayCtrl0Spec> {
        RegRoughSelOCebW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Rough delay select for output Clock Negative"]
    #[inline(always)]
    pub fn reg_rough_sel_o_clk_n(&mut self) -> RegRoughSelOClkNW<'_, PsramRoughDelayCtrl0Spec> {
        RegRoughSelOClkNW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Rough delay select for output Clock Positive"]
    #[inline(always)]
    pub fn reg_rough_sel_o_clk(&mut self) -> RegRoughSelOClkW<'_, PsramRoughDelayCtrl0Spec> {
        RegRoughSelOClkW::new(self, 24)
    }
}
#[doc = "PSRAM Rough Delay Control Register 0 (DQS_OEN0, CEB, CLK_N, CLK)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_rough_delay_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_rough_delay_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramRoughDelayCtrl0Spec;
impl crate::RegisterSpec for PsramRoughDelayCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_rough_delay_ctrl0::R`](R) reader structure"]
impl crate::Readable for PsramRoughDelayCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_rough_delay_ctrl0::W`](W) writer structure"]
impl crate::Writable for PsramRoughDelayCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_ROUGH_DELAY_CTRL0 to value 0"]
impl crate::Resettable for PsramRoughDelayCtrl0Spec {}
