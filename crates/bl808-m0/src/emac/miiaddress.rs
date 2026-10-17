#[doc = "Register `MIIADDRESS` reader"]
pub type R = crate::R<MiiaddressSpec>;
#[doc = "Register `MIIADDRESS` writer"]
pub type W = crate::W<MiiaddressSpec>;
#[doc = "Field `FIAD` reader - PHY Address"]
pub type FiadR = crate::FieldReader;
#[doc = "Field `FIAD` writer - PHY Address"]
pub type FiadW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RGAD` reader - Register Address in PHY"]
pub type RgadR = crate::FieldReader;
#[doc = "Field `RGAD` writer - Register Address in PHY"]
pub type RgadW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - PHY Address"]
    #[inline(always)]
    pub fn fiad(&self) -> FiadR {
        FiadR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Register Address in PHY"]
    #[inline(always)]
    pub fn rgad(&self) -> RgadR {
        RgadR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - PHY Address"]
    #[inline(always)]
    pub fn fiad(&mut self) -> FiadW<'_, MiiaddressSpec> {
        FiadW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Register Address in PHY"]
    #[inline(always)]
    pub fn rgad(&mut self) -> RgadW<'_, MiiaddressSpec> {
        RgadW::new(self, 8)
    }
}
#[doc = "MII Management Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`miiaddress::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miiaddress::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiiaddressSpec;
impl crate::RegisterSpec for MiiaddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`miiaddress::R`](R) reader structure"]
impl crate::Readable for MiiaddressSpec {}
#[doc = "`write(|w| ..)` method takes [`miiaddress::W`](W) writer structure"]
impl crate::Writable for MiiaddressSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIIADDRESS to value 0"]
impl crate::Resettable for MiiaddressSpec {}
