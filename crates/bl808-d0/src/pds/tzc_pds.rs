#[doc = "Register `TZC_PDS` reader"]
pub type R = crate::R<TzcPdsSpec>;
#[doc = "Register `TZC_PDS` writer"]
pub type W = crate::W<TzcPdsSpec>;
#[doc = "Field `cr_e902_cfg_wr_lock` reader - "]
pub type CrE902CfgWrLockR = crate::BitReader;
#[doc = "Field `cr_e902_cfg_wr_lock` writer - "]
pub type CrE902CfgWrLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_e906_cfg_wr_lock` reader - "]
pub type CrE906CfgWrLockR = crate::BitReader;
#[doc = "Field `cr_e906_cfg_wr_lock` writer - "]
pub type CrE906CfgWrLockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_e902_cfg_wr_lock(&self) -> CrE902CfgWrLockR {
        CrE902CfgWrLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_e906_cfg_wr_lock(&self) -> CrE906CfgWrLockR {
        CrE906CfgWrLockR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_e902_cfg_wr_lock(&mut self) -> CrE902CfgWrLockW<'_, TzcPdsSpec> {
        CrE902CfgWrLockW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_e906_cfg_wr_lock(&mut self) -> CrE906CfgWrLockW<'_, TzcPdsSpec> {
        CrE906CfgWrLockW::new(self, 1)
    }
}
#[doc = "TrustZone write lock for CPU config registers\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_pds::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_pds::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcPdsSpec;
impl crate::RegisterSpec for TzcPdsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_pds::R`](R) reader structure"]
impl crate::Readable for TzcPdsSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_pds::W`](W) writer structure"]
impl crate::Writable for TzcPdsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_PDS to value 0"]
impl crate::Resettable for TzcPdsSpec {}
