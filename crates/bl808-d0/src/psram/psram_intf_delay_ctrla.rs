#[doc = "Register `PSRAM_INTF_DELAY_CTRLA` reader"]
pub type R = crate::R<PsramIntfDelayCtrlaSpec>;
#[doc = "Register `PSRAM_INTF_DELAY_CTRLA` writer"]
pub type W = crate::W<PsramIntfDelayCtrlaSpec>;
#[doc = "Field `reg_delay_sel_i_dqs1` reader - Delay select for input DQS pin 1"]
pub type RegDelaySelIDqs1R = crate::FieldReader<u16>;
#[doc = "Field `reg_delay_sel_i_dqs1` writer - Delay select for input DQS pin 1"]
pub type RegDelaySelIDqs1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `reg_delay_sel_i_adq15` reader - Delay select for input ADQ pin 15"]
pub type RegDelaySelIAdq15R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_i_adq15` writer - Delay select for input ADQ pin 15"]
pub type RegDelaySelIAdq15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_i_adq14` reader - Delay select for input ADQ pin 14"]
pub type RegDelaySelIAdq14R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_i_adq14` writer - Delay select for input ADQ pin 14"]
pub type RegDelaySelIAdq14W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - Delay select for input DQS pin 1"]
    #[inline(always)]
    pub fn reg_delay_sel_i_dqs1(&self) -> RegDelaySelIDqs1R {
        RegDelaySelIDqs1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Delay select for input ADQ pin 15"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq15(&self) -> RegDelaySelIAdq15R {
        RegDelaySelIAdq15R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Delay select for input ADQ pin 14"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq14(&self) -> RegDelaySelIAdq14R {
        RegDelaySelIAdq14R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Delay select for input DQS pin 1"]
    #[inline(always)]
    pub fn reg_delay_sel_i_dqs1(&mut self) -> RegDelaySelIDqs1W<'_, PsramIntfDelayCtrlaSpec> {
        RegDelaySelIDqs1W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Delay select for input ADQ pin 15"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq15(&mut self) -> RegDelaySelIAdq15W<'_, PsramIntfDelayCtrlaSpec> {
        RegDelaySelIAdq15W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Delay select for input ADQ pin 14"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq14(&mut self) -> RegDelaySelIAdq14W<'_, PsramIntfDelayCtrlaSpec> {
        RegDelaySelIAdq14W::new(self, 24)
    }
}
#[doc = "PSRAM Interface Fine Delay Control Register A (DQS1 In, ADQ15-14 In)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_intf_delay_ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_intf_delay_ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramIntfDelayCtrlaSpec;
impl crate::RegisterSpec for PsramIntfDelayCtrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_intf_delay_ctrla::R`](R) reader structure"]
impl crate::Readable for PsramIntfDelayCtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`psram_intf_delay_ctrla::W`](W) writer structure"]
impl crate::Writable for PsramIntfDelayCtrlaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_INTF_DELAY_CTRLA to value 0"]
impl crate::Resettable for PsramIntfDelayCtrlaSpec {}
