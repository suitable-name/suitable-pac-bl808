#[doc = "Register `padc_oec_w1p` reader"]
pub type R = crate::R<PadcOecW1pSpec>;
#[doc = "Register `padc_oec_w1p` writer"]
pub type W = crate::W<PadcOecW1pSpec>;
#[doc = "Field `padc_wr_oec` reader - "]
pub type PadcWrOecR = crate::FieldReader<u32>;
#[doc = "Field `padc_wr_oec` writer - "]
pub type PadcWrOecW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `padc_oec_w1p_g` writer - "]
pub type PadcOecW1pGW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_wr_oec(&self) -> PadcWrOecR {
        PadcWrOecR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_wr_oec(&mut self) -> PadcWrOecW<'_, PadcOecW1pSpec> {
        PadcWrOecW::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn padc_oec_w1p_g(&mut self) -> PadcOecW1pGW<'_, PadcOecW1pSpec> {
        PadcOecW1pGW::new(self, 24)
    }
}
#[doc = "PADC Offset Error Correction W1P Trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_oec_w1p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_oec_w1p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcOecW1pSpec;
impl crate::RegisterSpec for PadcOecW1pSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_oec_w1p::R`](R) reader structure"]
impl crate::Readable for PadcOecW1pSpec {}
#[doc = "`write(|w| ..)` method takes [`padc_oec_w1p::W`](W) writer structure"]
impl crate::Writable for PadcOecW1pSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets padc_oec_w1p to value 0"]
impl crate::Resettable for PadcOecW1pSpec {}
