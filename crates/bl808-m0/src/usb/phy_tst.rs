#[doc = "Register `PHY_TST` reader"]
pub type R = crate::R<PhyTstSpec>;
#[doc = "Register `PHY_TST` writer"]
pub type W = crate::W<PhyTstSpec>;
#[doc = "Field `UNPLUG` reader - Unplug signal"]
pub type UnplugR = crate::BitReader;
#[doc = "Field `UNPLUG` writer - Unplug signal"]
pub type UnplugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TST_JSTA` reader - Test J state"]
pub type TstJstaR = crate::BitReader;
#[doc = "Field `TST_JSTA` writer - Test J state"]
pub type TstJstaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TST_KSTA` reader - Test K state"]
pub type TstKstaR = crate::BitReader;
#[doc = "Field `TST_KSTA` writer - Test K state"]
pub type TstKstaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TST_SE0NAK` reader - Test SE0 NAK"]
pub type TstSe0nakR = crate::BitReader;
#[doc = "Field `TST_SE0NAK` writer - Test SE0 NAK"]
pub type TstSe0nakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TST_PKT` reader - Test Packet"]
pub type TstPktR = crate::BitReader;
#[doc = "Field `TST_PKT` writer - Test Packet"]
pub type TstPktW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Unplug signal"]
    #[inline(always)]
    pub fn unplug(&self) -> UnplugR {
        UnplugR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Test J state"]
    #[inline(always)]
    pub fn tst_jsta(&self) -> TstJstaR {
        TstJstaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Test K state"]
    #[inline(always)]
    pub fn tst_ksta(&self) -> TstKstaR {
        TstKstaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Test SE0 NAK"]
    #[inline(always)]
    pub fn tst_se0nak(&self) -> TstSe0nakR {
        TstSe0nakR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Test Packet"]
    #[inline(always)]
    pub fn tst_pkt(&self) -> TstPktR {
        TstPktR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Unplug signal"]
    #[inline(always)]
    pub fn unplug(&mut self) -> UnplugW<'_, PhyTstSpec> {
        UnplugW::new(self, 0)
    }
    #[doc = "Bit 1 - Test J state"]
    #[inline(always)]
    pub fn tst_jsta(&mut self) -> TstJstaW<'_, PhyTstSpec> {
        TstJstaW::new(self, 1)
    }
    #[doc = "Bit 2 - Test K state"]
    #[inline(always)]
    pub fn tst_ksta(&mut self) -> TstKstaW<'_, PhyTstSpec> {
        TstKstaW::new(self, 2)
    }
    #[doc = "Bit 3 - Test SE0 NAK"]
    #[inline(always)]
    pub fn tst_se0nak(&mut self) -> TstSe0nakW<'_, PhyTstSpec> {
        TstSe0nakW::new(self, 3)
    }
    #[doc = "Bit 4 - Test Packet"]
    #[inline(always)]
    pub fn tst_pkt(&mut self) -> TstPktW<'_, PhyTstSpec> {
        TstPktW::new(self, 4)
    }
}
#[doc = "PHY Test Control\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_tst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_tst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyTstSpec;
impl crate::RegisterSpec for PhyTstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_tst::R`](R) reader structure"]
impl crate::Readable for PhyTstSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_tst::W`](W) writer structure"]
impl crate::Writable for PhyTstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_TST to value 0"]
impl crate::Resettable for PhyTstSpec {}
