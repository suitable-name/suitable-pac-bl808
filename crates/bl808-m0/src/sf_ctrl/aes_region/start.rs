#[doc = "Register `START` reader"]
pub type R = crate::R<StartSpec>;
#[doc = "Register `START` writer"]
pub type W = crate::W<StartSpec>;
#[doc = "Field `addr_start` reader - "]
pub type AddrStartR = crate::FieldReader<u32>;
#[doc = "Field `addr_start` writer - "]
pub type AddrStartW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `hw_key_en` reader - "]
pub type HwKeyEnR = crate::BitReader;
#[doc = "Field `hw_key_en` writer - "]
pub type HwKeyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `en` reader - "]
pub type EnR = crate::BitReader;
#[doc = "Field `en` writer - "]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lock` reader - "]
pub type LockR = crate::BitReader;
#[doc = "Field `lock` writer - "]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn addr_start(&self) -> AddrStartR {
        AddrStartR::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn hw_key_en(&self) -> HwKeyEnR {
        HwKeyEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn addr_start(&mut self) -> AddrStartW<'_, StartSpec> {
        AddrStartW::new(self, 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn hw_key_en(&mut self) -> HwKeyEnW<'_, StartSpec> {
        HwKeyEnW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, StartSpec> {
        EnW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, StartSpec> {
        LockW::new(self, 31)
    }
}
#[doc = "Region Start Address and Config\n\nYou can [`read`](crate::Reg::read) this register and get [`start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StartSpec;
impl crate::RegisterSpec for StartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`start::R`](R) reader structure"]
impl crate::Readable for StartSpec {}
#[doc = "`write(|w| ..)` method takes [`start::W`](W) writer structure"]
impl crate::Writable for StartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets START to value 0"]
impl crate::Resettable for StartSpec {}
