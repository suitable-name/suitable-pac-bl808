#[doc = "Register `DEV_CXCFG` reader"]
pub type R = crate::R<DevCxcfgSpec>;
#[doc = "Register `DEV_CXCFG` writer"]
pub type W = crate::W<DevCxcfgSpec>;
#[doc = "Field `VSTA_HOV` reader - VBUS Status (Host Overwrite)"]
pub type VstaHovR = crate::FieldReader;
#[doc = "Field `VSTA_HOV` writer - VBUS Status (Host Overwrite)"]
pub type VstaHovW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - VBUS Status (Host Overwrite)"]
    #[inline(always)]
    pub fn vsta_hov(&self) -> VstaHovR {
        VstaHovR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - VBUS Status (Host Overwrite)"]
    #[inline(always)]
    pub fn vsta_hov(&mut self) -> VstaHovW<'_, DevCxcfgSpec> {
        VstaHovW::new(self, 0)
    }
}
#[doc = "Device Control Endpoint Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_cxcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_cxcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevCxcfgSpec;
impl crate::RegisterSpec for DevCxcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_cxcfg::R`](R) reader structure"]
impl crate::Readable for DevCxcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dev_cxcfg::W`](W) writer structure"]
impl crate::Writable for DevCxcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_CXCFG to value 0"]
impl crate::Resettable for DevCxcfgSpec {}
