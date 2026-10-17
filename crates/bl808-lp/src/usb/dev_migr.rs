#[doc = "Register `DEV_MIGR` reader"]
pub type R = crate::R<DevMigrSpec>;
#[doc = "Register `DEV_MIGR` writer"]
pub type W = crate::W<DevMigrSpec>;
#[doc = "Field `MINT_G0` reader - "]
pub type MintG0R = crate::BitReader;
#[doc = "Field `MINT_G0` writer - "]
pub type MintG0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINT_G1` reader - "]
pub type MintG1R = crate::BitReader;
#[doc = "Field `MINT_G1` writer - "]
pub type MintG1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINT_G2` reader - "]
pub type MintG2R = crate::BitReader;
#[doc = "Field `MINT_G2` writer - "]
pub type MintG2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINT_G3` reader - "]
pub type MintG3R = crate::BitReader;
#[doc = "Field `MINT_G3` writer - "]
pub type MintG3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINT_G4` reader - "]
pub type MintG4R = crate::BitReader;
#[doc = "Field `MINT_G4` writer - "]
pub type MintG4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mint_g0(&self) -> MintG0R {
        MintG0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mint_g1(&self) -> MintG1R {
        MintG1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn mint_g2(&self) -> MintG2R {
        MintG2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn mint_g3(&self) -> MintG3R {
        MintG3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mint_g4(&self) -> MintG4R {
        MintG4R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mint_g0(&mut self) -> MintG0W<'_, DevMigrSpec> {
        MintG0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mint_g1(&mut self) -> MintG1W<'_, DevMigrSpec> {
        MintG1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn mint_g2(&mut self) -> MintG2W<'_, DevMigrSpec> {
        MintG2W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn mint_g3(&mut self) -> MintG3W<'_, DevMigrSpec> {
        MintG3W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mint_g4(&mut self) -> MintG4W<'_, DevMigrSpec> {
        MintG4W::new(self, 4)
    }
}
#[doc = "Device Master Interrupt Group Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_migr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_migr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevMigrSpec;
impl crate::RegisterSpec for DevMigrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_migr::R`](R) reader structure"]
impl crate::Readable for DevMigrSpec {}
#[doc = "`write(|w| ..)` method takes [`dev_migr::W`](W) writer structure"]
impl crate::Writable for DevMigrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_MIGR to value 0"]
impl crate::Resettable for DevMigrSpec {}
