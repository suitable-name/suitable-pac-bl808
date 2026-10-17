#[doc = "Register `PHY_CFG_4C` reader"]
pub type R = crate::R<PhyCfg4cSpec>;
#[doc = "Register `PHY_CFG_4C` writer"]
pub type W = crate::W<PhyCfg4cSpec>;
#[doc = "Field `tout_uhs_phy_dig` reader - Digital PHY test output"]
pub type ToutUhsPhyDigR = crate::FieldReader<u16>;
#[doc = "Field `odt_sel_dly` reader - ODT selection delay"]
pub type OdtSelDlyR = crate::FieldReader;
#[doc = "Field `odt_sel_dly` writer - ODT selection delay"]
pub type OdtSelDlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `odt_sel_hw` reader - Hardware ODT selection"]
pub type OdtSelHwR = crate::BitReader;
#[doc = "Field `odt_sel_hw` writer - Hardware ODT selection"]
pub type OdtSelHwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Digital PHY test output"]
    #[inline(always)]
    pub fn tout_uhs_phy_dig(&self) -> ToutUhsPhyDigR {
        ToutUhsPhyDigR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - ODT selection delay"]
    #[inline(always)]
    pub fn odt_sel_dly(&self) -> OdtSelDlyR {
        OdtSelDlyR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Hardware ODT selection"]
    #[inline(always)]
    pub fn odt_sel_hw(&self) -> OdtSelHwR {
        OdtSelHwR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:19 - ODT selection delay"]
    #[inline(always)]
    pub fn odt_sel_dly(&mut self) -> OdtSelDlyW<'_, PhyCfg4cSpec> {
        OdtSelDlyW::new(self, 16)
    }
    #[doc = "Bit 20 - Hardware ODT selection"]
    #[inline(always)]
    pub fn odt_sel_hw(&mut self) -> OdtSelHwW<'_, PhyCfg4cSpec> {
        OdtSelHwW::new(self, 20)
    }
}
#[doc = "PHY ODT and Digital Output Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_4c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_4c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyCfg4cSpec;
impl crate::RegisterSpec for PhyCfg4cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_cfg_4c::R`](R) reader structure"]
impl crate::Readable for PhyCfg4cSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_cfg_4c::W`](W) writer structure"]
impl crate::Writable for PhyCfg4cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_CFG_4C to value 0x0013_0000"]
impl crate::Resettable for PhyCfg4cSpec {
    const RESET_VALUE: u32 = 0x0013_0000;
}
