#[doc = "Register `BMX_CFG3` reader"]
pub type R = crate::R<BmxCfg3Spec>;
#[doc = "Register `BMX_CFG3` writer"]
pub type W = crate::W<BmxCfg3Spec>;
#[doc = "Field `reg_bmx_berr_clr` reader - "]
pub type RegBmxBerrClrR = crate::BitReader;
#[doc = "Field `reg_bmx_berr_clr` writer - "]
pub type RegBmxBerrClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_berr_last` reader - "]
pub type RegBmxBerrLastR = crate::BitReader;
#[doc = "Field `reg_mcu_berr_clr` reader - "]
pub type RegMcuBerrClrR = crate::BitReader;
#[doc = "Field `reg_mcu_berr_clr` writer - "]
pub type RegMcuBerrClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mcu_berr_last` reader - "]
pub type RegMcuBerrLastR = crate::BitReader;
#[doc = "Field `sts_bmx_berr` reader - "]
pub type StsBmxBerrR = crate::BitReader;
#[doc = "Field `sts_mcu_berr` reader - "]
pub type StsMcuBerrR = crate::BitReader;
#[doc = "Field `sts_bmx_berr_write` reader - "]
pub type StsBmxBerrWriteR = crate::BitReader;
#[doc = "Field `sts_mcu_berr_write` reader - "]
pub type StsMcuBerrWriteR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_bmx_berr_clr(&self) -> RegBmxBerrClrR {
        RegBmxBerrClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_bmx_berr_last(&self) -> RegBmxBerrLastR {
        RegBmxBerrLastR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_mcu_berr_clr(&self) -> RegMcuBerrClrR {
        RegMcuBerrClrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_mcu_berr_last(&self) -> RegMcuBerrLastR {
        RegMcuBerrLastR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sts_bmx_berr(&self) -> StsBmxBerrR {
        StsBmxBerrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sts_mcu_berr(&self) -> StsMcuBerrR {
        StsMcuBerrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sts_bmx_berr_write(&self) -> StsBmxBerrWriteR {
        StsBmxBerrWriteR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sts_mcu_berr_write(&self) -> StsMcuBerrWriteR {
        StsMcuBerrWriteR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_bmx_berr_clr(&mut self) -> RegBmxBerrClrW<'_, BmxCfg3Spec> {
        RegBmxBerrClrW::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_mcu_berr_clr(&mut self) -> RegMcuBerrClrW<'_, BmxCfg3Spec> {
        RegMcuBerrClrW::new(self, 8)
    }
}
#[doc = "BMX Bus Matrix Config 3 (Bus Error Clear/Status)\n\nYou can [`read`](crate::Reg::read) this register and get [`bmx_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmx_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmxCfg3Spec;
impl crate::RegisterSpec for BmxCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmx_cfg3::R`](R) reader structure"]
impl crate::Readable for BmxCfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`bmx_cfg3::W`](W) writer structure"]
impl crate::Writable for BmxCfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BMX_CFG3 to value 0"]
impl crate::Resettable for BmxCfg3Spec {}
