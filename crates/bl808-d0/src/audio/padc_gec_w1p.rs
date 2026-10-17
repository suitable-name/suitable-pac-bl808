#[doc = "Register `padc_gec_w1p` reader"]
pub type R = crate::R<PadcGecW1pSpec>;
#[doc = "Register `padc_gec_w1p` writer"]
pub type W = crate::W<PadcGecW1pSpec>;
#[doc = "Field `padc_wr_gec` reader - "]
pub type PadcWrGecR = crate::FieldReader<u32>;
#[doc = "Field `padc_wr_gec` writer - "]
pub type PadcWrGecW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `padc_gec_w1p_g` writer - "]
pub type PadcGecW1pGW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_wr_gec(&self) -> PadcWrGecR {
        PadcWrGecR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_wr_gec(&mut self) -> PadcWrGecW<'_, PadcGecW1pSpec> {
        PadcWrGecW::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn padc_gec_w1p_g(&mut self) -> PadcGecW1pGW<'_, PadcGecW1pSpec> {
        PadcGecW1pGW::new(self, 24)
    }
}
#[doc = "PADC Gain Error Correction W1P Trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_gec_w1p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_gec_w1p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcGecW1pSpec;
impl crate::RegisterSpec for PadcGecW1pSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_gec_w1p::R`](R) reader structure"]
impl crate::Readable for PadcGecW1pSpec {}
#[doc = "`write(|w| ..)` method takes [`padc_gec_w1p::W`](W) writer structure"]
impl crate::Writable for PadcGecW1pSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets padc_gec_w1p to value 0"]
impl crate::Resettable for PadcGecW1pSpec {}
