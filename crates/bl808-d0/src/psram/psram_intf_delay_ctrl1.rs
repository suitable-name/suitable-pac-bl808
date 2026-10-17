#[doc = "Register `PSRAM_INTF_DELAY_CTRL1` reader"]
pub type R = crate::R<PsramIntfDelayCtrl1Spec>;
#[doc = "Register `PSRAM_INTF_DELAY_CTRL1` writer"]
pub type W = crate::W<PsramIntfDelayCtrl1Spec>;
#[doc = "Field `reg_delay_sel_o_adq1` reader - Delay select for output ADQ pin 1"]
pub type RegDelaySelOAdq1R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_adq1` writer - Delay select for output ADQ pin 1"]
pub type RegDelaySelOAdq1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_o_adq0` reader - Delay select for output ADQ pin 0"]
pub type RegDelaySelOAdq0R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_adq0` writer - Delay select for output ADQ pin 0"]
pub type RegDelaySelOAdq0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_o_adq_oen0` reader - Delay select for output ADQ enable for slice 0"]
pub type RegDelaySelOAdqOen0R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_adq_oen0` writer - Delay select for output ADQ enable for slice 0"]
pub type RegDelaySelOAdqOen0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_o_dqs0` reader - Delay select for output DQS pin 0"]
pub type RegDelaySelODqs0R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_dqs0` writer - Delay select for output DQS pin 0"]
pub type RegDelaySelODqs0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Delay select for output ADQ pin 1"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq1(&self) -> RegDelaySelOAdq1R {
        RegDelaySelOAdq1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Delay select for output ADQ pin 0"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq0(&self) -> RegDelaySelOAdq0R {
        RegDelaySelOAdq0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay select for output ADQ enable for slice 0"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq_oen0(&self) -> RegDelaySelOAdqOen0R {
        RegDelaySelOAdqOen0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Delay select for output DQS pin 0"]
    #[inline(always)]
    pub fn reg_delay_sel_o_dqs0(&self) -> RegDelaySelODqs0R {
        RegDelaySelODqs0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Delay select for output ADQ pin 1"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq1(&mut self) -> RegDelaySelOAdq1W<'_, PsramIntfDelayCtrl1Spec> {
        RegDelaySelOAdq1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Delay select for output ADQ pin 0"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq0(&mut self) -> RegDelaySelOAdq0W<'_, PsramIntfDelayCtrl1Spec> {
        RegDelaySelOAdq0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Delay select for output ADQ enable for slice 0"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq_oen0(
        &mut self,
    ) -> RegDelaySelOAdqOen0W<'_, PsramIntfDelayCtrl1Spec> {
        RegDelaySelOAdqOen0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Delay select for output DQS pin 0"]
    #[inline(always)]
    pub fn reg_delay_sel_o_dqs0(&mut self) -> RegDelaySelODqs0W<'_, PsramIntfDelayCtrl1Spec> {
        RegDelaySelODqs0W::new(self, 24)
    }
}
#[doc = "PSRAM Interface Fine Delay Control Register 1 (ADQ1, ADQ0, ADQ_OEN0, DQS0)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_intf_delay_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_intf_delay_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramIntfDelayCtrl1Spec;
impl crate::RegisterSpec for PsramIntfDelayCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_intf_delay_ctrl1::R`](R) reader structure"]
impl crate::Readable for PsramIntfDelayCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_intf_delay_ctrl1::W`](W) writer structure"]
impl crate::Writable for PsramIntfDelayCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_INTF_DELAY_CTRL1 to value 0"]
impl crate::Resettable for PsramIntfDelayCtrl1Spec {}
