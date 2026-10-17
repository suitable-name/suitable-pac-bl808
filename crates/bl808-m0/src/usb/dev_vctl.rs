#[doc = "Register `DEV_VCTL` reader"]
pub type R = crate::R<DevVctlSpec>;
#[doc = "Register `DEV_VCTL` writer"]
pub type W = crate::W<DevVctlSpec>;
#[doc = "Field `VCTL_HOV` reader - VBUS Control (Host Overwrite)"]
pub type VctlHovR = crate::FieldReader;
#[doc = "Field `VCTL_HOV` writer - VBUS Control (Host Overwrite)"]
pub type VctlHovW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VCTLOAD_N_HOV` reader - VBUS Control Load (Host Overwrite)"]
pub type VctloadNHovR = crate::BitReader;
#[doc = "Field `VCTLOAD_N_HOV` writer - VBUS Control Load (Host Overwrite)"]
pub type VctloadNHovW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - VBUS Control (Host Overwrite)"]
    #[inline(always)]
    pub fn vctl_hov(&self) -> VctlHovR {
        VctlHovR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - VBUS Control Load (Host Overwrite)"]
    #[inline(always)]
    pub fn vctload_n_hov(&self) -> VctloadNHovR {
        VctloadNHovR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - VBUS Control (Host Overwrite)"]
    #[inline(always)]
    pub fn vctl_hov(&mut self) -> VctlHovW<'_, DevVctlSpec> {
        VctlHovW::new(self, 0)
    }
    #[doc = "Bit 5 - VBUS Control Load (Host Overwrite)"]
    #[inline(always)]
    pub fn vctload_n_hov(&mut self) -> VctloadNHovW<'_, DevVctlSpec> {
        VctloadNHovW::new(self, 5)
    }
}
#[doc = "Device VBUS Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_vctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_vctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevVctlSpec;
impl crate::RegisterSpec for DevVctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_vctl::R`](R) reader structure"]
impl crate::Readable for DevVctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dev_vctl::W`](W) writer structure"]
impl crate::Writable for DevVctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_VCTL to value 0"]
impl crate::Resettable for DevVctlSpec {}
