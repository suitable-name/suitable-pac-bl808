#[doc = "Register `BLAI_BIAS_ADDR` reader"]
pub type R = crate::R<BlaiBiasAddrSpec>;
#[doc = "Register `BLAI_BIAS_ADDR` writer"]
pub type W = crate::W<BlaiBiasAddrSpec>;
#[doc = "Field `reg_bias_addr_base` reader - Base address for bias data."]
pub type RegBiasAddrBaseR = crate::FieldReader<u32>;
#[doc = "Field `reg_bias_addr_base` writer - Base address for bias data."]
pub type RegBiasAddrBaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Base address for bias data."]
    #[inline(always)]
    pub fn reg_bias_addr_base(&self) -> RegBiasAddrBaseR {
        RegBiasAddrBaseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address for bias data."]
    #[inline(always)]
    pub fn reg_bias_addr_base(&mut self) -> RegBiasAddrBaseW<'_, BlaiBiasAddrSpec> {
        RegBiasAddrBaseW::new(self, 0)
    }
}
#[doc = "Bias Data Base Address Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_bias_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_bias_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiBiasAddrSpec;
impl crate::RegisterSpec for BlaiBiasAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_bias_addr::R`](R) reader structure"]
impl crate::Readable for BlaiBiasAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`blai_bias_addr::W`](W) writer structure"]
impl crate::Writable for BlaiBiasAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_BIAS_ADDR to value 0"]
impl crate::Resettable for BlaiBiasAddrSpec {}
