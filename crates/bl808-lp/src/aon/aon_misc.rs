#[doc = "Register `AON_MISC` reader"]
pub type R = crate::R<AonMiscSpec>;
#[doc = "Register `AON_MISC` writer"]
pub type W = crate::W<AonMiscSpec>;
#[doc = "Field `sw_soc_en_aon` reader - "]
pub type SwSocEnAonR = crate::BitReader;
#[doc = "Field `sw_soc_en_aon` writer - "]
pub type SwSocEnAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sw_wb_en_aon` reader - "]
pub type SwWbEnAonR = crate::BitReader;
#[doc = "Field `sw_wb_en_aon` writer - "]
pub type SwWbEnAonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sw_soc_en_aon(&self) -> SwSocEnAonR {
        SwSocEnAonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sw_wb_en_aon(&self) -> SwWbEnAonR {
        SwWbEnAonR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sw_soc_en_aon(&mut self) -> SwSocEnAonW<'_, AonMiscSpec> {
        SwSocEnAonW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sw_wb_en_aon(&mut self) -> SwWbEnAonW<'_, AonMiscSpec> {
        SwWbEnAonW::new(self, 1)
    }
}
#[doc = "AON Miscellaneous Controls\n\nYou can [`read`](crate::Reg::read) this register and get [`aon_misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon_misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonMiscSpec;
impl crate::RegisterSpec for AonMiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_misc::R`](R) reader structure"]
impl crate::Readable for AonMiscSpec {}
#[doc = "`write(|w| ..)` method takes [`aon_misc::W`](W) writer structure"]
impl crate::Writable for AonMiscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AON_MISC to value 0"]
impl crate::Resettable for AonMiscSpec {}
