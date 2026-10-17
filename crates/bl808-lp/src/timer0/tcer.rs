#[doc = "Register `TCER` reader"]
pub type R = crate::R<TcerSpec>;
#[doc = "Register `TCER` writer"]
pub type W = crate::W<TcerSpec>;
#[doc = "Field `timer2_en` reader - "]
pub type Timer2EnR = crate::BitReader;
#[doc = "Field `timer2_en` writer - "]
pub type Timer2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `timer3_en` reader - "]
pub type Timer3EnR = crate::BitReader;
#[doc = "Field `timer3_en` writer - "]
pub type Timer3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tcr2_cnt_clr` reader - "]
pub type Tcr2CntClrR = crate::BitReader;
#[doc = "Field `tcr2_cnt_clr` writer - "]
pub type Tcr2CntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tcr3_cnt_clr` reader - "]
pub type Tcr3CntClrR = crate::BitReader;
#[doc = "Field `tcr3_cnt_clr` writer - "]
pub type Tcr3CntClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer2_en(&self) -> Timer2EnR {
        Timer2EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer3_en(&self) -> Timer3EnR {
        Timer3EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tcr2_cnt_clr(&self) -> Tcr2CntClrR {
        Tcr2CntClrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tcr3_cnt_clr(&self) -> Tcr3CntClrR {
        Tcr3CntClrR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer2_en(&mut self) -> Timer2EnW<'_, TcerSpec> {
        Timer2EnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer3_en(&mut self) -> Timer3EnW<'_, TcerSpec> {
        Timer3EnW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tcr2_cnt_clr(&mut self) -> Tcr2CntClrW<'_, TcerSpec> {
        Tcr2CntClrW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tcr3_cnt_clr(&mut self) -> Tcr3CntClrW<'_, TcerSpec> {
        Tcr3CntClrW::new(self, 6)
    }
}
#[doc = "Timer Counter Enable/Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`tcer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcerSpec;
impl crate::RegisterSpec for TcerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcer::R`](R) reader structure"]
impl crate::Readable for TcerSpec {}
#[doc = "`write(|w| ..)` method takes [`tcer::W`](W) writer structure"]
impl crate::Writable for TcerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCER to value 0"]
impl crate::Resettable for TcerSpec {}
