#[doc = "Register `TCDR_FORCE` reader"]
pub type R = crate::R<TcdrForceSpec>;
#[doc = "Register `TCDR_FORCE` writer"]
pub type W = crate::W<TcdrForceSpec>;
#[doc = "Field `tcdr2_force` reader - "]
pub type Tcdr2ForceR = crate::BitReader;
#[doc = "Field `tcdr2_force` writer - "]
pub type Tcdr2ForceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tcdr3_force` reader - "]
pub type Tcdr3ForceR = crate::BitReader;
#[doc = "Field `tcdr3_force` writer - "]
pub type Tcdr3ForceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wcdr_force` reader - "]
pub type WcdrForceR = crate::BitReader;
#[doc = "Field `wcdr_force` writer - "]
pub type WcdrForceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tcdr2_force(&self) -> Tcdr2ForceR {
        Tcdr2ForceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tcdr3_force(&self) -> Tcdr3ForceR {
        Tcdr3ForceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wcdr_force(&self) -> WcdrForceR {
        WcdrForceR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tcdr2_force(&mut self) -> Tcdr2ForceW<'_, TcdrForceSpec> {
        Tcdr2ForceW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tcdr3_force(&mut self) -> Tcdr3ForceW<'_, TcdrForceSpec> {
        Tcdr3ForceW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wcdr_force(&mut self) -> WcdrForceW<'_, TcdrForceSpec> {
        WcdrForceW::new(self, 4)
    }
}
#[doc = "Timer Division Force Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`tcdr_force::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcdr_force::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcdrForceSpec;
impl crate::RegisterSpec for TcdrForceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcdr_force::R`](R) reader structure"]
impl crate::Readable for TcdrForceSpec {}
#[doc = "`write(|w| ..)` method takes [`tcdr_force::W`](W) writer structure"]
impl crate::Writable for TcdrForceSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCDR_FORCE to value 0"]
impl crate::Resettable for TcdrForceSpec {}
