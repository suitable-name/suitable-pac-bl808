#[doc = "Register `PSRAM_ROUGH_DELAY_CTRL1` reader"]
pub type R = crate::R<PsramRoughDelayCtrl1Spec>;
#[doc = "Register `PSRAM_ROUGH_DELAY_CTRL1` writer"]
pub type W = crate::W<PsramRoughDelayCtrl1Spec>;
#[doc = "Field `reg_rough_sel_o_adq1` reader - Rough delay select for output ADQ pin 1"]
pub type RegRoughSelOAdq1R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_adq1` writer - Rough delay select for output ADQ pin 1"]
pub type RegRoughSelOAdq1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_o_adq0` reader - Rough delay select for output ADQ pin 0"]
pub type RegRoughSelOAdq0R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_adq0` writer - Rough delay select for output ADQ pin 0"]
pub type RegRoughSelOAdq0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_o_adq_oen0` reader - Rough delay select for output ADQ enable for slice 0"]
pub type RegRoughSelOAdqOen0R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_adq_oen0` writer - Rough delay select for output ADQ enable for slice 0"]
pub type RegRoughSelOAdqOen0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_o_dqs0` reader - Rough delay select for output DQS pin 0"]
pub type RegRoughSelODqs0R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_dqs0` writer - Rough delay select for output DQS pin 0"]
pub type RegRoughSelODqs0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Rough delay select for output ADQ pin 1"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq1(&self) -> RegRoughSelOAdq1R {
        RegRoughSelOAdq1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Rough delay select for output ADQ pin 0"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq0(&self) -> RegRoughSelOAdq0R {
        RegRoughSelOAdq0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Rough delay select for output ADQ enable for slice 0"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq_oen0(&self) -> RegRoughSelOAdqOen0R {
        RegRoughSelOAdqOen0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Rough delay select for output DQS pin 0"]
    #[inline(always)]
    pub fn reg_rough_sel_o_dqs0(&self) -> RegRoughSelODqs0R {
        RegRoughSelODqs0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Rough delay select for output ADQ pin 1"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq1(&mut self) -> RegRoughSelOAdq1W<'_, PsramRoughDelayCtrl1Spec> {
        RegRoughSelOAdq1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Rough delay select for output ADQ pin 0"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq0(&mut self) -> RegRoughSelOAdq0W<'_, PsramRoughDelayCtrl1Spec> {
        RegRoughSelOAdq0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Rough delay select for output ADQ enable for slice 0"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq_oen0(
        &mut self,
    ) -> RegRoughSelOAdqOen0W<'_, PsramRoughDelayCtrl1Spec> {
        RegRoughSelOAdqOen0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Rough delay select for output DQS pin 0"]
    #[inline(always)]
    pub fn reg_rough_sel_o_dqs0(&mut self) -> RegRoughSelODqs0W<'_, PsramRoughDelayCtrl1Spec> {
        RegRoughSelODqs0W::new(self, 24)
    }
}
#[doc = "PSRAM Rough Delay Control Register 1 (ADQ1, ADQ0, ADQ_OEN0, DQS0)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_rough_delay_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_rough_delay_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramRoughDelayCtrl1Spec;
impl crate::RegisterSpec for PsramRoughDelayCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_rough_delay_ctrl1::R`](R) reader structure"]
impl crate::Readable for PsramRoughDelayCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_rough_delay_ctrl1::W`](W) writer structure"]
impl crate::Writable for PsramRoughDelayCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_ROUGH_DELAY_CTRL1 to value 0"]
impl crate::Resettable for PsramRoughDelayCtrl1Spec {}
