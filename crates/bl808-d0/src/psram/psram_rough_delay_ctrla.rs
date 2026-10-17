#[doc = "Register `PSRAM_ROUGH_DELAY_CTRLA` reader"]
pub type R = crate::R<PsramRoughDelayCtrlaSpec>;
#[doc = "Register `PSRAM_ROUGH_DELAY_CTRLA` writer"]
pub type W = crate::W<PsramRoughDelayCtrlaSpec>;
#[doc = "Field `reg_rough_sel_i_dqs1` reader - Rough delay select for input DQS pin 1"]
pub type RegRoughSelIDqs1R = crate::FieldReader<u16>;
#[doc = "Field `reg_rough_sel_i_dqs1` writer - Rough delay select for input DQS pin 1"]
pub type RegRoughSelIDqs1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `reg_rough_sel_i_adq15` reader - Rough delay select for input ADQ pin 15"]
pub type RegRoughSelIAdq15R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_i_adq15` writer - Rough delay select for input ADQ pin 15"]
pub type RegRoughSelIAdq15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_i_adq14` reader - Rough delay select for input ADQ pin 14"]
pub type RegRoughSelIAdq14R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_i_adq14` writer - Rough delay select for input ADQ pin 14"]
pub type RegRoughSelIAdq14W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - Rough delay select for input DQS pin 1"]
    #[inline(always)]
    pub fn reg_rough_sel_i_dqs1(&self) -> RegRoughSelIDqs1R {
        RegRoughSelIDqs1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Rough delay select for input ADQ pin 15"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq15(&self) -> RegRoughSelIAdq15R {
        RegRoughSelIAdq15R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Rough delay select for input ADQ pin 14"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq14(&self) -> RegRoughSelIAdq14R {
        RegRoughSelIAdq14R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Rough delay select for input DQS pin 1"]
    #[inline(always)]
    pub fn reg_rough_sel_i_dqs1(&mut self) -> RegRoughSelIDqs1W<'_, PsramRoughDelayCtrlaSpec> {
        RegRoughSelIDqs1W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Rough delay select for input ADQ pin 15"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq15(&mut self) -> RegRoughSelIAdq15W<'_, PsramRoughDelayCtrlaSpec> {
        RegRoughSelIAdq15W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Rough delay select for input ADQ pin 14"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq14(&mut self) -> RegRoughSelIAdq14W<'_, PsramRoughDelayCtrlaSpec> {
        RegRoughSelIAdq14W::new(self, 24)
    }
}
#[doc = "PSRAM Rough Delay Control Register A (DQS1 In, ADQ15-14 In)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_rough_delay_ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_rough_delay_ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramRoughDelayCtrlaSpec;
impl crate::RegisterSpec for PsramRoughDelayCtrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_rough_delay_ctrla::R`](R) reader structure"]
impl crate::Readable for PsramRoughDelayCtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`psram_rough_delay_ctrla::W`](W) writer structure"]
impl crate::Writable for PsramRoughDelayCtrlaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_ROUGH_DELAY_CTRLA to value 0"]
impl crate::Resettable for PsramRoughDelayCtrlaSpec {}
