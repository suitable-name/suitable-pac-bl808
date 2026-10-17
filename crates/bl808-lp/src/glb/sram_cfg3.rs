#[doc = "Register `SRAM_CFG3` reader"]
pub type R = crate::R<SramCfg3Spec>;
#[doc = "Register `SRAM_CFG3` writer"]
pub type W = crate::W<SramCfg3Spec>;
#[doc = "Field `em_sel` reader - "]
pub type EmSelR = crate::FieldReader;
#[doc = "Field `em_sel` writer - "]
pub type EmSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_vram_sel` reader - "]
pub type RegVramSelR = crate::FieldReader;
#[doc = "Field `reg_vram_sel` writer - "]
pub type RegVramSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn em_sel(&self) -> EmSelR {
        EmSelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn reg_vram_sel(&self) -> RegVramSelR {
        RegVramSelR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn em_sel(&mut self) -> EmSelW<'_, SramCfg3Spec> {
        EmSelW::new(self, 0)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn reg_vram_sel(&mut self) -> RegVramSelW<'_, SramCfg3Spec> {
        RegVramSelW::new(self, 28)
    }
}
#[doc = "SRAM Configuration 3 - EM and VRAM Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramCfg3Spec;
impl crate::RegisterSpec for SramCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_cfg3::R`](R) reader structure"]
impl crate::Readable for SramCfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`sram_cfg3::W`](W) writer structure"]
impl crate::Writable for SramCfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM_CFG3 to value 0"]
impl crate::Resettable for SramCfg3Spec {}
