#[doc = "Register `MIIMODE` reader"]
pub type R = crate::R<MiimodeSpec>;
#[doc = "Register `MIIMODE` writer"]
pub type W = crate::W<MiimodeSpec>;
#[doc = "Field `CLKDIV` reader - "]
pub type ClkdivR = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - "]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MIINOPRE` reader - "]
pub type MiinopreR = crate::BitReader;
#[doc = "Field `MIINOPRE` writer - "]
pub type MiinopreW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn miinopre(&self) -> MiinopreR {
        MiinopreR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> ClkdivW<'_, MiimodeSpec> {
        ClkdivW::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn miinopre(&mut self) -> MiinopreW<'_, MiimodeSpec> {
        MiinopreW::new(self, 8)
    }
}
#[doc = "MII Management Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`miimode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miimode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiimodeSpec;
impl crate::RegisterSpec for MiimodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`miimode::R`](R) reader structure"]
impl crate::Readable for MiimodeSpec {}
#[doc = "`write(|w| ..)` method takes [`miimode::W`](W) writer structure"]
impl crate::Writable for MiimodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIIMODE to value 0x64"]
impl crate::Resettable for MiimodeSpec {
    const RESET_VALUE: u32 = 0x64;
}
