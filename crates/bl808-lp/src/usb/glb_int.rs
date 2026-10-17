#[doc = "Register `GLB_INT` reader"]
pub type R = crate::R<GlbIntSpec>;
#[doc = "Register `GLB_INT` writer"]
pub type W = crate::W<GlbIntSpec>;
#[doc = "Field `MDEV_INT` reader - Device interrupt mask"]
pub type MdevIntR = crate::BitReader;
#[doc = "Field `MDEV_INT` writer - Device interrupt mask"]
pub type MdevIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOTG_INT` reader - OTG interrupt mask"]
pub type MotgIntR = crate::BitReader;
#[doc = "Field `MOTG_INT` writer - OTG interrupt mask"]
pub type MotgIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MHC_INT` reader - Host Controller interrupt mask"]
pub type MhcIntR = crate::BitReader;
#[doc = "Field `MHC_INT` writer - Host Controller interrupt mask"]
pub type MhcIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_POLARITY` reader - Interrupt signal polarity"]
pub type IntPolarityR = crate::BitReader;
#[doc = "Field `INT_POLARITY` writer - Interrupt signal polarity"]
pub type IntPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Device interrupt mask"]
    #[inline(always)]
    pub fn mdev_int(&self) -> MdevIntR {
        MdevIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OTG interrupt mask"]
    #[inline(always)]
    pub fn motg_int(&self) -> MotgIntR {
        MotgIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Host Controller interrupt mask"]
    #[inline(always)]
    pub fn mhc_int(&self) -> MhcIntR {
        MhcIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt signal polarity"]
    #[inline(always)]
    pub fn int_polarity(&self) -> IntPolarityR {
        IntPolarityR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Device interrupt mask"]
    #[inline(always)]
    pub fn mdev_int(&mut self) -> MdevIntW<'_, GlbIntSpec> {
        MdevIntW::new(self, 0)
    }
    #[doc = "Bit 1 - OTG interrupt mask"]
    #[inline(always)]
    pub fn motg_int(&mut self) -> MotgIntW<'_, GlbIntSpec> {
        MotgIntW::new(self, 1)
    }
    #[doc = "Bit 2 - Host Controller interrupt mask"]
    #[inline(always)]
    pub fn mhc_int(&mut self) -> MhcIntW<'_, GlbIntSpec> {
        MhcIntW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt signal polarity"]
    #[inline(always)]
    pub fn int_polarity(&mut self) -> IntPolarityW<'_, GlbIntSpec> {
        IntPolarityW::new(self, 3)
    }
}
#[doc = "Global Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`glb_int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`glb_int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlbIntSpec;
impl crate::RegisterSpec for GlbIntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`glb_int::R`](R) reader structure"]
impl crate::Readable for GlbIntSpec {}
#[doc = "`write(|w| ..)` method takes [`glb_int::W`](W) writer structure"]
impl crate::Writable for GlbIntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GLB_INT to value 0"]
impl crate::Resettable for GlbIntSpec {}
