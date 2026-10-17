#[doc = "Register `SE_CDET_0_CTRL_0` reader"]
pub type R = crate::R<SeCdet0Ctrl0Spec>;
#[doc = "Register `SE_CDET_0_CTRL_0` writer"]
pub type W = crate::W<SeCdet0Ctrl0Spec>;
#[doc = "Field `se_cdet_0_en` reader - "]
pub type SeCdet0EnR = crate::BitReader;
#[doc = "Field `se_cdet_0_en` writer - "]
pub type SeCdet0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_cdet_0_error` reader - "]
pub type SeCdet0ErrorR = crate::BitReader;
#[doc = "Field `se_cdet_0_status` reader - "]
pub type SeCdet0StatusR = crate::FieldReader<u16>;
#[doc = "Field `se_cdet_0_g_loop_max` reader - "]
pub type SeCdet0GLoopMaxR = crate::FieldReader;
#[doc = "Field `se_cdet_0_g_loop_max` writer - "]
pub type SeCdet0GLoopMaxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `se_cdet_0_g_loop_min` reader - "]
pub type SeCdet0GLoopMinR = crate::FieldReader;
#[doc = "Field `se_cdet_0_g_loop_min` writer - "]
pub type SeCdet0GLoopMinW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_cdet_0_en(&self) -> SeCdet0EnR {
        SeCdet0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_cdet_0_error(&self) -> SeCdet0ErrorR {
        SeCdet0ErrorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:15"]
    #[inline(always)]
    pub fn se_cdet_0_status(&self) -> SeCdet0StatusR {
        SeCdet0StatusR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn se_cdet_0_g_loop_max(&self) -> SeCdet0GLoopMaxR {
        SeCdet0GLoopMaxR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn se_cdet_0_g_loop_min(&self) -> SeCdet0GLoopMinR {
        SeCdet0GLoopMinR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_cdet_0_en(&mut self) -> SeCdet0EnW<'_, SeCdet0Ctrl0Spec> {
        SeCdet0EnW::new(self, 0)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn se_cdet_0_g_loop_max(&mut self) -> SeCdet0GLoopMaxW<'_, SeCdet0Ctrl0Spec> {
        SeCdet0GLoopMaxW::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn se_cdet_0_g_loop_min(&mut self) -> SeCdet0GLoopMinW<'_, SeCdet0Ctrl0Spec> {
        SeCdet0GLoopMinW::new(self, 24)
    }
}
#[doc = "Clock Detection Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`se_cdet_0_ctrl_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_cdet_0_ctrl_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeCdet0Ctrl0Spec;
impl crate::RegisterSpec for SeCdet0Ctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_cdet_0_ctrl_0::R`](R) reader structure"]
impl crate::Readable for SeCdet0Ctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`se_cdet_0_ctrl_0::W`](W) writer structure"]
impl crate::Writable for SeCdet0Ctrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_CDET_0_CTRL_0 to value 0x2164_0004"]
impl crate::Resettable for SeCdet0Ctrl0Spec {
    const RESET_VALUE: u32 = 0x2164_0004;
}
