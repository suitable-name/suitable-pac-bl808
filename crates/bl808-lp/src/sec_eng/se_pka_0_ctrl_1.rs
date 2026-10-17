#[doc = "Register `SE_PKA_0_CTRL_1` reader"]
pub type R = crate::R<SePka0Ctrl1Spec>;
#[doc = "Register `SE_PKA_0_CTRL_1` writer"]
pub type W = crate::W<SePka0Ctrl1Spec>;
#[doc = "Field `se_pka_0_hburst` reader - "]
pub type SePka0HburstR = crate::FieldReader;
#[doc = "Field `se_pka_0_hburst` writer - "]
pub type SePka0HburstW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `se_pka_0_hbypass` reader - "]
pub type SePka0HbypassR = crate::BitReader;
#[doc = "Field `se_pka_0_hbypass` writer - "]
pub type SePka0HbypassW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn se_pka_0_hburst(&self) -> SePka0HburstR {
        SePka0HburstR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_pka_0_hbypass(&self) -> SePka0HbypassR {
        SePka0HbypassR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn se_pka_0_hburst(&mut self) -> SePka0HburstW<'_, SePka0Ctrl1Spec> {
        SePka0HburstW::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_pka_0_hbypass(&mut self) -> SePka0HbypassW<'_, SePka0Ctrl1Spec> {
        SePka0HbypassW::new(self, 3)
    }
}
#[doc = "PKA Burst Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`se_pka_0_ctrl_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_pka_0_ctrl_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SePka0Ctrl1Spec;
impl crate::RegisterSpec for SePka0Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_pka_0_ctrl_1::R`](R) reader structure"]
impl crate::Readable for SePka0Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`se_pka_0_ctrl_1::W`](W) writer structure"]
impl crate::Writable for SePka0Ctrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_PKA_0_CTRL_1 to value 0x05"]
impl crate::Resettable for SePka0Ctrl1Spec {
    const RESET_VALUE: u32 = 0x05;
}
