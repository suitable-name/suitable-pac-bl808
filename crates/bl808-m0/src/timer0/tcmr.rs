#[doc = "Register `TCMR` reader"]
pub type R = crate::R<TcmrSpec>;
#[doc = "Register `TCMR` writer"]
pub type W = crate::W<TcmrSpec>;
#[doc = "Field `timer2_mode` reader - "]
pub type Timer2ModeR = crate::BitReader;
#[doc = "Field `timer2_mode` writer - "]
pub type Timer2ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `timer3_mode` reader - "]
pub type Timer3ModeR = crate::BitReader;
#[doc = "Field `timer3_mode` writer - "]
pub type Timer3ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `timer2_align` reader - "]
pub type Timer2AlignR = crate::BitReader;
#[doc = "Field `timer2_align` writer - "]
pub type Timer2AlignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `timer3_align` reader - "]
pub type Timer3AlignR = crate::BitReader;
#[doc = "Field `timer3_align` writer - "]
pub type Timer3AlignW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer2_mode(&self) -> Timer2ModeR {
        Timer2ModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer3_mode(&self) -> Timer3ModeR {
        Timer3ModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn timer2_align(&self) -> Timer2AlignR {
        Timer2AlignR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn timer3_align(&self) -> Timer3AlignR {
        Timer3AlignR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer2_mode(&mut self) -> Timer2ModeW<'_, TcmrSpec> {
        Timer2ModeW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer3_mode(&mut self) -> Timer3ModeW<'_, TcmrSpec> {
        Timer3ModeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn timer2_align(&mut self) -> Timer2AlignW<'_, TcmrSpec> {
        Timer2AlignW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn timer3_align(&mut self) -> Timer3AlignW<'_, TcmrSpec> {
        Timer3AlignW::new(self, 6)
    }
}
#[doc = "Timer Counter Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`tcmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcmrSpec;
impl crate::RegisterSpec for TcmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcmr::R`](R) reader structure"]
impl crate::Readable for TcmrSpec {}
#[doc = "`write(|w| ..)` method takes [`tcmr::W`](W) writer structure"]
impl crate::Writable for TcmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCMR to value 0"]
impl crate::Resettable for TcmrSpec {}
