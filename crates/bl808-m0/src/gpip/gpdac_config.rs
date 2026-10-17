#[doc = "Register `GPDAC_CONFIG` reader"]
pub type R = crate::R<GpdacConfigSpec>;
#[doc = "Register `GPDAC_CONFIG` writer"]
pub type W = crate::W<GpdacConfigSpec>;
#[doc = "Field `gpdac_en` reader - GPDAC enable"]
pub type GpdacEnR = crate::BitReader;
#[doc = "Field `gpdac_en` writer - GPDAC enable"]
pub type GpdacEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_mode` reader - DAC Mode: 0:32k, 1:16k, 2:8k, 3:512k (DMA only)"]
pub type GpdacModeR = crate::FieldReader;
#[doc = "Field `gpdac_mode` writer - DAC Mode: 0:32k, 1:16k, 2:8k, 3:512k (DMA only)"]
pub type GpdacModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpdac_ch_a_sel` reader - Channel A Source Select (0:Reg, 1:DMA, etc.)"]
pub type GpdacChASelR = crate::FieldReader;
#[doc = "Field `gpdac_ch_a_sel` writer - Channel A Source Select (0:Reg, 1:DMA, etc.)"]
pub type GpdacChASelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `gpdac_ch_b_sel` reader - Channel B Source Select"]
pub type GpdacChBSelR = crate::FieldReader;
#[doc = "Field `gpdac_ch_b_sel` writer - Channel B Source Select"]
pub type GpdacChBSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - GPDAC enable"]
    #[inline(always)]
    pub fn gpdac_en(&self) -> GpdacEnR {
        GpdacEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - DAC Mode: 0:32k, 1:16k, 2:8k, 3:512k (DMA only)"]
    #[inline(always)]
    pub fn gpdac_mode(&self) -> GpdacModeR {
        GpdacModeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Channel A Source Select (0:Reg, 1:DMA, etc.)"]
    #[inline(always)]
    pub fn gpdac_ch_a_sel(&self) -> GpdacChASelR {
        GpdacChASelR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Channel B Source Select"]
    #[inline(always)]
    pub fn gpdac_ch_b_sel(&self) -> GpdacChBSelR {
        GpdacChBSelR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - GPDAC enable"]
    #[inline(always)]
    pub fn gpdac_en(&mut self) -> GpdacEnW<'_, GpdacConfigSpec> {
        GpdacEnW::new(self, 0)
    }
    #[doc = "Bits 8:10 - DAC Mode: 0:32k, 1:16k, 2:8k, 3:512k (DMA only)"]
    #[inline(always)]
    pub fn gpdac_mode(&mut self) -> GpdacModeW<'_, GpdacConfigSpec> {
        GpdacModeW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Channel A Source Select (0:Reg, 1:DMA, etc.)"]
    #[inline(always)]
    pub fn gpdac_ch_a_sel(&mut self) -> GpdacChASelW<'_, GpdacConfigSpec> {
        GpdacChASelW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Channel B Source Select"]
    #[inline(always)]
    pub fn gpdac_ch_b_sel(&mut self) -> GpdacChBSelW<'_, GpdacConfigSpec> {
        GpdacChBSelW::new(self, 20)
    }
}
#[doc = "GPDAC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpdac_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdac_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpdacConfigSpec;
impl crate::RegisterSpec for GpdacConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdac_config::R`](R) reader structure"]
impl crate::Readable for GpdacConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`gpdac_config::W`](W) writer structure"]
impl crate::Writable for GpdacConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPDAC_CONFIG to value 0x0d00_0000"]
impl crate::Resettable for GpdacConfigSpec {
    const RESET_VALUE: u32 = 0x0d00_0000;
}
