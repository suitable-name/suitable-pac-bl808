#[doc = "Register `BMX_CFG1` reader"]
pub type R = crate::R<BmxCfg1Spec>;
#[doc = "Register `BMX_CFG1` writer"]
pub type W = crate::W<BmxCfg1Spec>;
#[doc = "Field `reg_bmx_berr_int_en` reader - "]
pub type RegBmxBerrIntEnR = crate::BitReader;
#[doc = "Field `reg_bmx_berr_int_en` writer - "]
pub type RegBmxBerrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mcu_berr_int_en` reader - "]
pub type RegMcuBerrIntEnR = crate::BitReader;
#[doc = "Field `reg_mcu_berr_int_en` writer - "]
pub type RegMcuBerrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_qos_cpu` reader - "]
pub type RegBmxQosCpuR = crate::BitReader;
#[doc = "Field `reg_bmx_qos_cpu` writer - "]
pub type RegBmxQosCpuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_qos_sdu` reader - "]
pub type RegBmxQosSduR = crate::BitReader;
#[doc = "Field `reg_bmx_qos_sdu` writer - "]
pub type RegBmxQosSduW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_qos_sec0` reader - "]
pub type RegBmxQosSec0R = crate::BitReader;
#[doc = "Field `reg_bmx_qos_sec0` writer - "]
pub type RegBmxQosSec0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_qos_sec1` reader - "]
pub type RegBmxQosSec1R = crate::BitReader;
#[doc = "Field `reg_bmx_qos_sec1` writer - "]
pub type RegBmxQosSec1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_qos_sec2` reader - "]
pub type RegBmxQosSec2R = crate::BitReader;
#[doc = "Field `reg_bmx_qos_sec2` writer - "]
pub type RegBmxQosSec2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_qos_dma` reader - "]
pub type RegBmxQosDmaR = crate::BitReader;
#[doc = "Field `reg_bmx_qos_dma` writer - "]
pub type RegBmxQosDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_qos_cci` reader - "]
pub type RegBmxQosCciR = crate::BitReader;
#[doc = "Field `reg_bmx_qos_cci` writer - "]
pub type RegBmxQosCciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_qos_pldma` reader - "]
pub type RegBmxQosPldmaR = crate::BitReader;
#[doc = "Field `reg_bmx_qos_pldma` writer - "]
pub type RegBmxQosPldmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_qos_blem` reader - "]
pub type RegBmxQosBlemR = crate::BitReader;
#[doc = "Field `reg_bmx_qos_blem` writer - "]
pub type RegBmxQosBlemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_qos_emaca` reader - "]
pub type RegBmxQosEmacaR = crate::BitReader;
#[doc = "Field `reg_bmx_qos_emaca` writer - "]
pub type RegBmxQosEmacaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_qos_dma2` reader - "]
pub type RegBmxQosDma2R = crate::BitReader;
#[doc = "Field `reg_bmx_qos_dma2` writer - "]
pub type RegBmxQosDma2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_qos_sdhm` reader - "]
pub type RegBmxQosSdhmR = crate::BitReader;
#[doc = "Field `reg_bmx_qos_sdhm` writer - "]
pub type RegBmxQosSdhmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bmx_dbg_sel` reader - "]
pub type BmxDbgSelR = crate::FieldReader;
#[doc = "Field `bmx_dbg_sel` writer - "]
pub type BmxDbgSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_bmx_berr_int_en(&self) -> RegBmxBerrIntEnR {
        RegBmxBerrIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_mcu_berr_int_en(&self) -> RegMcuBerrIntEnR {
        RegMcuBerrIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_bmx_qos_cpu(&self) -> RegBmxQosCpuR {
        RegBmxQosCpuR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_bmx_qos_sdu(&self) -> RegBmxQosSduR {
        RegBmxQosSduR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg_bmx_qos_sec0(&self) -> RegBmxQosSec0R {
        RegBmxQosSec0R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg_bmx_qos_sec1(&self) -> RegBmxQosSec1R {
        RegBmxQosSec1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_bmx_qos_sec2(&self) -> RegBmxQosSec2R {
        RegBmxQosSec2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_bmx_qos_dma(&self) -> RegBmxQosDmaR {
        RegBmxQosDmaR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn reg_bmx_qos_cci(&self) -> RegBmxQosCciR {
        RegBmxQosCciR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reg_bmx_qos_pldma(&self) -> RegBmxQosPldmaR {
        RegBmxQosPldmaR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn reg_bmx_qos_blem(&self) -> RegBmxQosBlemR {
        RegBmxQosBlemR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn reg_bmx_qos_emaca(&self) -> RegBmxQosEmacaR {
        RegBmxQosEmacaR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn reg_bmx_qos_dma2(&self) -> RegBmxQosDma2R {
        RegBmxQosDma2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reg_bmx_qos_sdhm(&self) -> RegBmxQosSdhmR {
        RegBmxQosSdhmR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn bmx_dbg_sel(&self) -> BmxDbgSelR {
        BmxDbgSelR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_bmx_berr_int_en(&mut self) -> RegBmxBerrIntEnW<'_, BmxCfg1Spec> {
        RegBmxBerrIntEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_mcu_berr_int_en(&mut self) -> RegMcuBerrIntEnW<'_, BmxCfg1Spec> {
        RegMcuBerrIntEnW::new(self, 1)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_bmx_qos_cpu(&mut self) -> RegBmxQosCpuW<'_, BmxCfg1Spec> {
        RegBmxQosCpuW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_bmx_qos_sdu(&mut self) -> RegBmxQosSduW<'_, BmxCfg1Spec> {
        RegBmxQosSduW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg_bmx_qos_sec0(&mut self) -> RegBmxQosSec0W<'_, BmxCfg1Spec> {
        RegBmxQosSec0W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg_bmx_qos_sec1(&mut self) -> RegBmxQosSec1W<'_, BmxCfg1Spec> {
        RegBmxQosSec1W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_bmx_qos_sec2(&mut self) -> RegBmxQosSec2W<'_, BmxCfg1Spec> {
        RegBmxQosSec2W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_bmx_qos_dma(&mut self) -> RegBmxQosDmaW<'_, BmxCfg1Spec> {
        RegBmxQosDmaW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn reg_bmx_qos_cci(&mut self) -> RegBmxQosCciW<'_, BmxCfg1Spec> {
        RegBmxQosCciW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reg_bmx_qos_pldma(&mut self) -> RegBmxQosPldmaW<'_, BmxCfg1Spec> {
        RegBmxQosPldmaW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn reg_bmx_qos_blem(&mut self) -> RegBmxQosBlemW<'_, BmxCfg1Spec> {
        RegBmxQosBlemW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn reg_bmx_qos_emaca(&mut self) -> RegBmxQosEmacaW<'_, BmxCfg1Spec> {
        RegBmxQosEmacaW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn reg_bmx_qos_dma2(&mut self) -> RegBmxQosDma2W<'_, BmxCfg1Spec> {
        RegBmxQosDma2W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reg_bmx_qos_sdhm(&mut self) -> RegBmxQosSdhmW<'_, BmxCfg1Spec> {
        RegBmxQosSdhmW::new(self, 27)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn bmx_dbg_sel(&mut self) -> BmxDbgSelW<'_, BmxCfg1Spec> {
        BmxDbgSelW::new(self, 28)
    }
}
#[doc = "BMX Bus Matrix Config 1 (Bus Error Interrupt Enable / QoS)\n\nYou can [`read`](crate::Reg::read) this register and get [`bmx_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmx_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmxCfg1Spec;
impl crate::RegisterSpec for BmxCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmx_cfg1::R`](R) reader structure"]
impl crate::Readable for BmxCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`bmx_cfg1::W`](W) writer structure"]
impl crate::Writable for BmxCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BMX_CFG1 to value 0"]
impl crate::Resettable for BmxCfg1Spec {}
