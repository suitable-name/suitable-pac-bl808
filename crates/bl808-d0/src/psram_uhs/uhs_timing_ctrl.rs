#[doc = "Register `UHS_TIMING_CTRL` reader"]
pub type R = crate::R<UhsTimingCtrlSpec>;
#[doc = "Register `UHS_TIMING_CTRL` writer"]
pub type W = crate::W<UhsTimingCtrlSpec>;
#[doc = "Field `reg_trc_cycle` reader - TRC cycle count"]
pub type RegTrcCycleR = crate::FieldReader;
#[doc = "Field `reg_trc_cycle` writer - TRC cycle count"]
pub type RegTrcCycleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_tcphr_cycle` reader - TCPHR cycle count"]
pub type RegTcphrCycleR = crate::FieldReader;
#[doc = "Field `reg_tcphr_cycle` writer - TCPHR cycle count"]
pub type RegTcphrCycleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_tcphw_cycle` reader - TCPHW cycle count"]
pub type RegTcphwCycleR = crate::FieldReader;
#[doc = "Field `reg_tcphw_cycle` writer - TCPHW cycle count"]
pub type RegTcphwCycleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_trfc_cycle` reader - TRFC cycle count"]
pub type RegTrfcCycleR = crate::FieldReader;
#[doc = "Field `reg_trfc_cycle` writer - TRFC cycle count"]
pub type RegTrfcCycleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TRC cycle count"]
    #[inline(always)]
    pub fn reg_trc_cycle(&self) -> RegTrcCycleR {
        RegTrcCycleR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TCPHR cycle count"]
    #[inline(always)]
    pub fn reg_tcphr_cycle(&self) -> RegTcphrCycleR {
        RegTcphrCycleR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - TCPHW cycle count"]
    #[inline(always)]
    pub fn reg_tcphw_cycle(&self) -> RegTcphwCycleR {
        RegTcphwCycleR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - TRFC cycle count"]
    #[inline(always)]
    pub fn reg_trfc_cycle(&self) -> RegTrfcCycleR {
        RegTrfcCycleR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TRC cycle count"]
    #[inline(always)]
    pub fn reg_trc_cycle(&mut self) -> RegTrcCycleW<'_, UhsTimingCtrlSpec> {
        RegTrcCycleW::new(self, 0)
    }
    #[doc = "Bits 8:15 - TCPHR cycle count"]
    #[inline(always)]
    pub fn reg_tcphr_cycle(&mut self) -> RegTcphrCycleW<'_, UhsTimingCtrlSpec> {
        RegTcphrCycleW::new(self, 8)
    }
    #[doc = "Bits 16:23 - TCPHW cycle count"]
    #[inline(always)]
    pub fn reg_tcphw_cycle(&mut self) -> RegTcphwCycleW<'_, UhsTimingCtrlSpec> {
        RegTcphwCycleW::new(self, 16)
    }
    #[doc = "Bits 24:31 - TRFC cycle count"]
    #[inline(always)]
    pub fn reg_trfc_cycle(&mut self) -> RegTrfcCycleW<'_, UhsTimingCtrlSpec> {
        RegTrfcCycleW::new(self, 24)
    }
}
#[doc = "UHS Hardware Timing Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_timing_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_timing_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsTimingCtrlSpec;
impl crate::RegisterSpec for UhsTimingCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_timing_ctrl::R`](R) reader structure"]
impl crate::Readable for UhsTimingCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`uhs_timing_ctrl::W`](W) writer structure"]
impl crate::Writable for UhsTimingCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHS_TIMING_CTRL to value 0"]
impl crate::Resettable for UhsTimingCtrlSpec {}
