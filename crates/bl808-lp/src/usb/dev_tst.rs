#[doc = "Register `DEV_TST` reader"]
pub type R = crate::R<DevTstSpec>;
#[doc = "Register `DEV_TST` writer"]
pub type W = crate::W<DevTstSpec>;
#[doc = "Field `TST_CLRFF_HOV` reader - Test Clear FIFO (Host Overwrite)"]
pub type TstClrffHovR = crate::BitReader;
#[doc = "Field `TST_CLRFF_HOV` writer - Test Clear FIFO (Host Overwrite)"]
pub type TstClrffHovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TST_LPCX` reader - Test Loop Control Endpoint X"]
pub type TstLpcxR = crate::BitReader;
#[doc = "Field `TST_LPCX` writer - Test Loop Control Endpoint X"]
pub type TstLpcxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TST_CLREA` reader - Test Clear All Endpoints"]
pub type TstClreaR = crate::BitReader;
#[doc = "Field `TST_CLREA` writer - Test Clear All Endpoints"]
pub type TstClreaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TST_DISTO_HOV` reader - Test Disable Timeout (Host Overwrite)"]
pub type TstDistoHovR = crate::BitReader;
#[doc = "Field `TST_DISTO_HOV` writer - Test Disable Timeout (Host Overwrite)"]
pub type TstDistoHovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TST_MOD_HOV` reader - Test Mode Enable (Host Overwrite)"]
pub type TstModHovR = crate::BitReader;
#[doc = "Field `TST_MOD_HOV` writer - Test Mode Enable (Host Overwrite)"]
pub type TstModHovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISGENSOF` reader - Disable Generate SOF"]
pub type DisgensofR = crate::BitReader;
#[doc = "Field `DISGENSOF` writer - Disable Generate SOF"]
pub type DisgensofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TST_MOD_TYP_HOV` reader - Test Mode Type (Host Overwrite)"]
pub type TstModTypHovR = crate::BitReader;
#[doc = "Field `TST_MOD_TYP_HOV` writer - Test Mode Type (Host Overwrite)"]
pub type TstModTypHovW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Test Clear FIFO (Host Overwrite)"]
    #[inline(always)]
    pub fn tst_clrff_hov(&self) -> TstClrffHovR {
        TstClrffHovR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Test Loop Control Endpoint X"]
    #[inline(always)]
    pub fn tst_lpcx(&self) -> TstLpcxR {
        TstLpcxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Test Clear All Endpoints"]
    #[inline(always)]
    pub fn tst_clrea(&self) -> TstClreaR {
        TstClreaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Test Disable Timeout (Host Overwrite)"]
    #[inline(always)]
    pub fn tst_disto_hov(&self) -> TstDistoHovR {
        TstDistoHovR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Test Mode Enable (Host Overwrite)"]
    #[inline(always)]
    pub fn tst_mod_hov(&self) -> TstModHovR {
        TstModHovR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable Generate SOF"]
    #[inline(always)]
    pub fn disgensof(&self) -> DisgensofR {
        DisgensofR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Test Mode Type (Host Overwrite)"]
    #[inline(always)]
    pub fn tst_mod_typ_hov(&self) -> TstModTypHovR {
        TstModTypHovR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Test Clear FIFO (Host Overwrite)"]
    #[inline(always)]
    pub fn tst_clrff_hov(&mut self) -> TstClrffHovW<'_, DevTstSpec> {
        TstClrffHovW::new(self, 0)
    }
    #[doc = "Bit 1 - Test Loop Control Endpoint X"]
    #[inline(always)]
    pub fn tst_lpcx(&mut self) -> TstLpcxW<'_, DevTstSpec> {
        TstLpcxW::new(self, 1)
    }
    #[doc = "Bit 2 - Test Clear All Endpoints"]
    #[inline(always)]
    pub fn tst_clrea(&mut self) -> TstClreaW<'_, DevTstSpec> {
        TstClreaW::new(self, 2)
    }
    #[doc = "Bit 4 - Test Disable Timeout (Host Overwrite)"]
    #[inline(always)]
    pub fn tst_disto_hov(&mut self) -> TstDistoHovW<'_, DevTstSpec> {
        TstDistoHovW::new(self, 4)
    }
    #[doc = "Bit 5 - Test Mode Enable (Host Overwrite)"]
    #[inline(always)]
    pub fn tst_mod_hov(&mut self) -> TstModHovW<'_, DevTstSpec> {
        TstModHovW::new(self, 5)
    }
    #[doc = "Bit 6 - Disable Generate SOF"]
    #[inline(always)]
    pub fn disgensof(&mut self) -> DisgensofW<'_, DevTstSpec> {
        DisgensofW::new(self, 6)
    }
    #[doc = "Bit 7 - Test Mode Type (Host Overwrite)"]
    #[inline(always)]
    pub fn tst_mod_typ_hov(&mut self) -> TstModTypHovW<'_, DevTstSpec> {
        TstModTypHovW::new(self, 7)
    }
}
#[doc = "Device Test Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_tst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_tst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevTstSpec;
impl crate::RegisterSpec for DevTstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_tst::R`](R) reader structure"]
impl crate::Readable for DevTstSpec {}
#[doc = "`write(|w| ..)` method takes [`dev_tst::W`](W) writer structure"]
impl crate::Writable for DevTstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_TST to value 0"]
impl crate::Resettable for DevTstSpec {}
