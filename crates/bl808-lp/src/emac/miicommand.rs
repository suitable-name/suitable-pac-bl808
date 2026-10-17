#[doc = "Register `MIICOMMAND` reader"]
pub type R = crate::R<MiicommandSpec>;
#[doc = "Register `MIICOMMAND` writer"]
pub type W = crate::W<MiicommandSpec>;
#[doc = "Field `SCANSTAT` reader - "]
pub type ScanstatR = crate::BitReader;
#[doc = "Field `SCANSTAT` writer - "]
pub type ScanstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTAT` reader - "]
pub type RstatR = crate::BitReader;
#[doc = "Field `RSTAT` writer - "]
pub type RstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCTRLDATA` reader - "]
pub type WctrldataR = crate::BitReader;
#[doc = "Field `WCTRLDATA` writer - "]
pub type WctrldataW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn scanstat(&self) -> ScanstatR {
        ScanstatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rstat(&self) -> RstatR {
        RstatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wctrldata(&self) -> WctrldataR {
        WctrldataR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn scanstat(&mut self) -> ScanstatW<'_, MiicommandSpec> {
        ScanstatW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rstat(&mut self) -> RstatW<'_, MiicommandSpec> {
        RstatW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wctrldata(&mut self) -> WctrldataW<'_, MiicommandSpec> {
        WctrldataW::new(self, 2)
    }
}
#[doc = "MII Management Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`miicommand::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miicommand::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiicommandSpec;
impl crate::RegisterSpec for MiicommandSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`miicommand::R`](R) reader structure"]
impl crate::Readable for MiicommandSpec {}
#[doc = "`write(|w| ..)` method takes [`miicommand::W`](W) writer structure"]
impl crate::Writable for MiicommandSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIICOMMAND to value 0"]
impl crate::Resettable for MiicommandSpec {}
