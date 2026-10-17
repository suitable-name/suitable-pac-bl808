#[doc = "Register `PDS_RAM2` reader"]
pub type R = crate::R<PdsRam2Spec>;
#[doc = "Register `PDS_RAM2` writer"]
pub type W = crate::W<PdsRam2Spec>;
#[doc = "Field `cr_wram_slp` reader - "]
pub type CrWramSlpR = crate::FieldReader<u16>;
#[doc = "Field `cr_wram_slp` writer - "]
pub type CrWramSlpW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `cr_wram_ret` reader - "]
pub type CrWramRetR = crate::FieldReader<u16>;
#[doc = "Field `cr_wram_ret` writer - "]
pub type CrWramRetW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn cr_wram_slp(&self) -> CrWramSlpR {
        CrWramSlpR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn cr_wram_ret(&self) -> CrWramRetR {
        CrWramRetR::new(((self.bits >> 10) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn cr_wram_slp(&mut self) -> CrWramSlpW<'_, PdsRam2Spec> {
        CrWramSlpW::new(self, 0)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn cr_wram_ret(&mut self) -> CrWramRetW<'_, PdsRam2Spec> {
        CrWramRetW::new(self, 10)
    }
}
#[doc = "PDS RAM Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_ram2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_ram2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdsRam2Spec;
impl crate::RegisterSpec for PdsRam2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_ram2::R`](R) reader structure"]
impl crate::Readable for PdsRam2Spec {}
#[doc = "`write(|w| ..)` method takes [`pds_ram2::W`](W) writer structure"]
impl crate::Writable for PdsRam2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDS_RAM2 to value 0"]
impl crate::Resettable for PdsRam2Spec {}
