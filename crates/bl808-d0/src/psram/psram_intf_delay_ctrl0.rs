#[doc = "Register `PSRAM_INTF_DELAY_CTRL0` reader"]
pub type R = crate::R<PsramIntfDelayCtrl0Spec>;
#[doc = "Register `PSRAM_INTF_DELAY_CTRL0` writer"]
pub type W = crate::W<PsramIntfDelayCtrl0Spec>;
#[doc = "Field `reg_delay_sel_o_dqs_oen0` reader - Delay select for output DQS enable for slice 0"]
pub type RegDelaySelODqsOen0R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_dqs_oen0` writer - Delay select for output DQS enable for slice 0"]
pub type RegDelaySelODqsOen0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_o_ceb` reader - Delay select for output Chip Enable"]
pub type RegDelaySelOCebR = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_ceb` writer - Delay select for output Chip Enable"]
pub type RegDelaySelOCebW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_o_clk_n` reader - Delay select for output Clock Negative"]
pub type RegDelaySelOClkNR = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_clk_n` writer - Delay select for output Clock Negative"]
pub type RegDelaySelOClkNW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_o_clk` reader - Delay select for output Clock Positive"]
pub type RegDelaySelOClkR = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_clk` writer - Delay select for output Clock Positive"]
pub type RegDelaySelOClkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Delay select for output DQS enable for slice 0"]
    #[inline(always)]
    pub fn reg_delay_sel_o_dqs_oen0(&self) -> RegDelaySelODqsOen0R {
        RegDelaySelODqsOen0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Delay select for output Chip Enable"]
    #[inline(always)]
    pub fn reg_delay_sel_o_ceb(&self) -> RegDelaySelOCebR {
        RegDelaySelOCebR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay select for output Clock Negative"]
    #[inline(always)]
    pub fn reg_delay_sel_o_clk_n(&self) -> RegDelaySelOClkNR {
        RegDelaySelOClkNR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Delay select for output Clock Positive"]
    #[inline(always)]
    pub fn reg_delay_sel_o_clk(&self) -> RegDelaySelOClkR {
        RegDelaySelOClkR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Delay select for output DQS enable for slice 0"]
    #[inline(always)]
    pub fn reg_delay_sel_o_dqs_oen0(
        &mut self,
    ) -> RegDelaySelODqsOen0W<'_, PsramIntfDelayCtrl0Spec> {
        RegDelaySelODqsOen0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Delay select for output Chip Enable"]
    #[inline(always)]
    pub fn reg_delay_sel_o_ceb(&mut self) -> RegDelaySelOCebW<'_, PsramIntfDelayCtrl0Spec> {
        RegDelaySelOCebW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Delay select for output Clock Negative"]
    #[inline(always)]
    pub fn reg_delay_sel_o_clk_n(&mut self) -> RegDelaySelOClkNW<'_, PsramIntfDelayCtrl0Spec> {
        RegDelaySelOClkNW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Delay select for output Clock Positive"]
    #[inline(always)]
    pub fn reg_delay_sel_o_clk(&mut self) -> RegDelaySelOClkW<'_, PsramIntfDelayCtrl0Spec> {
        RegDelaySelOClkW::new(self, 24)
    }
}
#[doc = "PSRAM Interface Fine Delay Control Register 0 (DQS_OEN0, CEB, CLK_N, CLK)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_intf_delay_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_intf_delay_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramIntfDelayCtrl0Spec;
impl crate::RegisterSpec for PsramIntfDelayCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_intf_delay_ctrl0::R`](R) reader structure"]
impl crate::Readable for PsramIntfDelayCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_intf_delay_ctrl0::W`](W) writer structure"]
impl crate::Writable for PsramIntfDelayCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_INTF_DELAY_CTRL0 to value 0"]
impl crate::Resettable for PsramIntfDelayCtrl0Spec {}
