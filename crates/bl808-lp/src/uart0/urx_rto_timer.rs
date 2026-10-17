#[doc = "Register `URX_RTO_TIMER` reader"]
pub type R = crate::R<UrxRtoTimerSpec>;
#[doc = "Register `URX_RTO_TIMER` writer"]
pub type W = crate::W<UrxRtoTimerSpec>;
#[doc = "Field `cr_urx_rto_value` reader - "]
pub type CrUrxRtoValueR = crate::FieldReader;
#[doc = "Field `cr_urx_rto_value` writer - "]
pub type CrUrxRtoValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_urx_rto_value(&self) -> CrUrxRtoValueR {
        CrUrxRtoValueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_urx_rto_value(&mut self) -> CrUrxRtoValueW<'_, UrxRtoTimerSpec> {
        CrUrxRtoValueW::new(self, 0)
    }
}
#[doc = "UART RX Timeout Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`urx_rto_timer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`urx_rto_timer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UrxRtoTimerSpec;
impl crate::RegisterSpec for UrxRtoTimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`urx_rto_timer::R`](R) reader structure"]
impl crate::Readable for UrxRtoTimerSpec {}
#[doc = "`write(|w| ..)` method takes [`urx_rto_timer::W`](W) writer structure"]
impl crate::Writable for UrxRtoTimerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets URX_RTO_TIMER to value 0x0f"]
impl crate::Resettable for UrxRtoTimerSpec {
    const RESET_VALUE: u32 = 0x0f;
}
