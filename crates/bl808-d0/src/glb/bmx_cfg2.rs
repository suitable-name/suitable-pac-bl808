#[doc = "Register `BMX_CFG2` reader"]
pub type R = crate::R<BmxCfg2Spec>;
#[doc = "Register `BMX_CFG2` writer"]
pub type W = crate::W<BmxCfg2Spec>;
#[doc = "Field `reg_bmx_berr_en` reader - "]
pub type RegBmxBerrEnR = crate::FieldReader<u16>;
#[doc = "Field `reg_bmx_berr_en` writer - "]
pub type RegBmxBerrEnW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `reg_mcu_berr_en` reader - "]
pub type RegMcuBerrEnR = crate::BitReader;
#[doc = "Field `reg_mcu_berr_en` writer - "]
pub type RegMcuBerrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn reg_bmx_berr_en(&self) -> RegBmxBerrEnR {
        RegBmxBerrEnR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_mcu_berr_en(&self) -> RegMcuBerrEnR {
        RegMcuBerrEnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn reg_bmx_berr_en(&mut self) -> RegBmxBerrEnW<'_, BmxCfg2Spec> {
        RegBmxBerrEnW::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_mcu_berr_en(&mut self) -> RegMcuBerrEnW<'_, BmxCfg2Spec> {
        RegMcuBerrEnW::new(self, 16)
    }
}
#[doc = "BMX Bus Matrix Config 2 (Bus Error Enable)\n\nYou can [`read`](crate::Reg::read) this register and get [`bmx_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmx_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmxCfg2Spec;
impl crate::RegisterSpec for BmxCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmx_cfg2::R`](R) reader structure"]
impl crate::Readable for BmxCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`bmx_cfg2::W`](W) writer structure"]
impl crate::Writable for BmxCfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BMX_CFG2 to value 0"]
impl crate::Resettable for BmxCfg2Spec {}
