#[doc = "Register `DVP2AXI_MISC` reader"]
pub type R = crate::R<Dvp2axiMiscSpec>;
#[doc = "Register `DVP2AXI_MISC` writer"]
pub type W = crate::W<Dvp2axiMiscSpec>;
#[doc = "Field `reg_alpha` reader - Alpha value for data expansion"]
pub type RegAlphaR = crate::FieldReader;
#[doc = "Field `reg_alpha` writer - Alpha value for data expansion"]
pub type RegAlphaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_format_565` reader - RGB565 format selection"]
pub type RegFormat565R = crate::FieldReader;
#[doc = "Field `reg_format_565` writer - RGB565 format selection"]
pub type RegFormat565W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - Alpha value for data expansion"]
    #[inline(always)]
    pub fn reg_alpha(&self) -> RegAlphaR {
        RegAlphaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - RGB565 format selection"]
    #[inline(always)]
    pub fn reg_format_565(&self) -> RegFormat565R {
        RegFormat565R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Alpha value for data expansion"]
    #[inline(always)]
    pub fn reg_alpha(&mut self) -> RegAlphaW<'_, Dvp2axiMiscSpec> {
        RegAlphaW::new(self, 0)
    }
    #[doc = "Bits 8:10 - RGB565 format selection"]
    #[inline(always)]
    pub fn reg_format_565(&mut self) -> RegFormat565W<'_, Dvp2axiMiscSpec> {
        RegFormat565W::new(self, 8)
    }
}
#[doc = "Miscellaneous Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp2axi_misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp2axi_misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dvp2axiMiscSpec;
impl crate::RegisterSpec for Dvp2axiMiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvp2axi_misc::R`](R) reader structure"]
impl crate::Readable for Dvp2axiMiscSpec {}
#[doc = "`write(|w| ..)` method takes [`dvp2axi_misc::W`](W) writer structure"]
impl crate::Writable for Dvp2axiMiscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DVP2AXI_MISC to value 0"]
impl crate::Resettable for Dvp2axiMiscSpec {}
