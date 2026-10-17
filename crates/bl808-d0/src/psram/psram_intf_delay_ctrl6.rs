#[doc = "Register `PSRAM_INTF_DELAY_CTRL6` reader"]
pub type R = crate::R<PsramIntfDelayCtrl6Spec>;
#[doc = "Register `PSRAM_INTF_DELAY_CTRL6` writer"]
pub type W = crate::W<PsramIntfDelayCtrl6Spec>;
#[doc = "Field `reg_delay_sel_o_adq9` reader - Delay select for output ADQ pin 9"]
pub type RegDelaySelOAdq9R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_adq9` writer - Delay select for output ADQ pin 9"]
pub type RegDelaySelOAdq9W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_o_adq8` reader - Delay select for output ADQ pin 8"]
pub type RegDelaySelOAdq8R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_adq8` writer - Delay select for output ADQ pin 8"]
pub type RegDelaySelOAdq8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_o_adq_oen1` reader - Delay select for output ADQ enable for slice 1"]
pub type RegDelaySelOAdqOen1R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_adq_oen1` writer - Delay select for output ADQ enable for slice 1"]
pub type RegDelaySelOAdqOen1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_o_dqs1` reader - Delay select for output DQS pin 1"]
pub type RegDelaySelODqs1R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_dqs1` writer - Delay select for output DQS pin 1"]
pub type RegDelaySelODqs1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Delay select for output ADQ pin 9"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq9(&self) -> RegDelaySelOAdq9R {
        RegDelaySelOAdq9R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Delay select for output ADQ pin 8"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq8(&self) -> RegDelaySelOAdq8R {
        RegDelaySelOAdq8R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay select for output ADQ enable for slice 1"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq_oen1(&self) -> RegDelaySelOAdqOen1R {
        RegDelaySelOAdqOen1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Delay select for output DQS pin 1"]
    #[inline(always)]
    pub fn reg_delay_sel_o_dqs1(&self) -> RegDelaySelODqs1R {
        RegDelaySelODqs1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Delay select for output ADQ pin 9"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq9(&mut self) -> RegDelaySelOAdq9W<'_, PsramIntfDelayCtrl6Spec> {
        RegDelaySelOAdq9W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Delay select for output ADQ pin 8"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq8(&mut self) -> RegDelaySelOAdq8W<'_, PsramIntfDelayCtrl6Spec> {
        RegDelaySelOAdq8W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Delay select for output ADQ enable for slice 1"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq_oen1(
        &mut self,
    ) -> RegDelaySelOAdqOen1W<'_, PsramIntfDelayCtrl6Spec> {
        RegDelaySelOAdqOen1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Delay select for output DQS pin 1"]
    #[inline(always)]
    pub fn reg_delay_sel_o_dqs1(&mut self) -> RegDelaySelODqs1W<'_, PsramIntfDelayCtrl6Spec> {
        RegDelaySelODqs1W::new(self, 24)
    }
}
#[doc = "PSRAM Interface Fine Delay Control Register 6 (ADQ9-8 Out, ADQ_OEN1, DQS1 Out)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_intf_delay_ctrl6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_intf_delay_ctrl6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramIntfDelayCtrl6Spec;
impl crate::RegisterSpec for PsramIntfDelayCtrl6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_intf_delay_ctrl6::R`](R) reader structure"]
impl crate::Readable for PsramIntfDelayCtrl6Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_intf_delay_ctrl6::W`](W) writer structure"]
impl crate::Writable for PsramIntfDelayCtrl6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_INTF_DELAY_CTRL6 to value 0"]
impl crate::Resettable for PsramIntfDelayCtrl6Spec {}
