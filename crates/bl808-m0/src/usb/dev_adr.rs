#[doc = "Register `DEV_ADR` reader"]
pub type R = crate::R<DevAdrSpec>;
#[doc = "Register `DEV_ADR` writer"]
pub type W = crate::W<DevAdrSpec>;
#[doc = "Field `DEVADR` reader - Device Address"]
pub type DevadrR = crate::FieldReader;
#[doc = "Field `DEVADR` writer - Device Address"]
pub type DevadrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `AFT_CONF` reader - Address After Configuration"]
pub type AftConfR = crate::BitReader;
#[doc = "Field `AFT_CONF` writer - Address After Configuration"]
pub type AftConfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Device Address"]
    #[inline(always)]
    pub fn devadr(&self) -> DevadrR {
        DevadrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Address After Configuration"]
    #[inline(always)]
    pub fn aft_conf(&self) -> AftConfR {
        AftConfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device Address"]
    #[inline(always)]
    pub fn devadr(&mut self) -> DevadrW<'_, DevAdrSpec> {
        DevadrW::new(self, 0)
    }
    #[doc = "Bit 7 - Address After Configuration"]
    #[inline(always)]
    pub fn aft_conf(&mut self) -> AftConfW<'_, DevAdrSpec> {
        AftConfW::new(self, 7)
    }
}
#[doc = "Device Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_adr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_adr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevAdrSpec;
impl crate::RegisterSpec for DevAdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_adr::R`](R) reader structure"]
impl crate::Readable for DevAdrSpec {}
#[doc = "`write(|w| ..)` method takes [`dev_adr::W`](W) writer structure"]
impl crate::Writable for DevAdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_ADR to value 0"]
impl crate::Resettable for DevAdrSpec {}
