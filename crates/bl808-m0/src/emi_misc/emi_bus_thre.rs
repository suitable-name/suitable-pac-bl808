#[doc = "Register `EMI_BUS_THRE` reader"]
pub type R = crate::R<EmiBusThreSpec>;
#[doc = "Register `EMI_BUS_THRE` writer"]
pub type W = crate::W<EmiBusThreSpec>;
#[doc = "Field `reg_x_wthre_pB` reader - "]
pub type RegXWthrePBR = crate::FieldReader;
#[doc = "Field `reg_x_wthre_pB` writer - "]
pub type RegXWthrePBW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_x_wthre_pA` reader - "]
pub type RegXWthrePAR = crate::FieldReader;
#[doc = "Field `reg_x_wthre_pA` writer - "]
pub type RegXWthrePAW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn reg_x_wthre_p_b(&self) -> RegXWthrePBR {
        RegXWthrePBR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_x_wthre_p_a(&self) -> RegXWthrePAR {
        RegXWthrePAR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn reg_x_wthre_p_b(&mut self) -> RegXWthrePBW<'_, EmiBusThreSpec> {
        RegXWthrePBW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_x_wthre_p_a(&mut self) -> RegXWthrePAW<'_, EmiBusThreSpec> {
        RegXWthrePAW::new(self, 2)
    }
}
#[doc = "EMI Bus Threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`emi_bus_thre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emi_bus_thre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmiBusThreSpec;
impl crate::RegisterSpec for EmiBusThreSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emi_bus_thre::R`](R) reader structure"]
impl crate::Readable for EmiBusThreSpec {}
#[doc = "`write(|w| ..)` method takes [`emi_bus_thre::W`](W) writer structure"]
impl crate::Writable for EmiBusThreSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMI_BUS_THRE to value 0"]
impl crate::Resettable for EmiBusThreSpec {}
