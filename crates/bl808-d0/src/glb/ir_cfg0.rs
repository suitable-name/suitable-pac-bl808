#[doc = "Register `IR_CFG0` reader"]
pub type R = crate::R<IrCfg0Spec>;
#[doc = "Register `IR_CFG0` writer"]
pub type W = crate::W<IrCfg0Spec>;
#[doc = "Field `ir_clk_div` reader - "]
pub type IrClkDivR = crate::FieldReader;
#[doc = "Field `ir_clk_div` writer - "]
pub type IrClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ir_clk_en` reader - "]
pub type IrClkEnR = crate::BitReader;
#[doc = "Field `ir_clk_en` writer - "]
pub type IrClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn ir_clk_div(&self) -> IrClkDivR {
        IrClkDivR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ir_clk_en(&self) -> IrClkEnR {
        IrClkEnR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn ir_clk_div(&mut self) -> IrClkDivW<'_, IrCfg0Spec> {
        IrClkDivW::new(self, 16)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ir_clk_en(&mut self) -> IrClkEnW<'_, IrCfg0Spec> {
        IrClkEnW::new(self, 23)
    }
}
#[doc = "IR/LED Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ir_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrCfg0Spec;
impl crate::RegisterSpec for IrCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir_cfg0::R`](R) reader structure"]
impl crate::Readable for IrCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ir_cfg0::W`](W) writer structure"]
impl crate::Writable for IrCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IR_CFG0 to value 0"]
impl crate::Resettable for IrCfg0Spec {}
