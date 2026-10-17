#[doc = "Register `PACKETLEN` reader"]
pub type R = crate::R<PacketlenSpec>;
#[doc = "Register `PACKETLEN` writer"]
pub type W = crate::W<PacketlenSpec>;
#[doc = "Field `MAXFL` reader - Maximum frame length"]
pub type MaxflR = crate::FieldReader<u16>;
#[doc = "Field `MAXFL` writer - Maximum frame length"]
pub type MaxflW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MINFL` reader - Minimum frame length"]
pub type MinflR = crate::FieldReader<u16>;
#[doc = "Field `MINFL` writer - Minimum frame length"]
pub type MinflW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Maximum frame length"]
    #[inline(always)]
    pub fn maxfl(&self) -> MaxflR {
        MaxflR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Minimum frame length"]
    #[inline(always)]
    pub fn minfl(&self) -> MinflR {
        MinflR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Maximum frame length"]
    #[inline(always)]
    pub fn maxfl(&mut self) -> MaxflW<'_, PacketlenSpec> {
        MaxflW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Minimum frame length"]
    #[inline(always)]
    pub fn minfl(&mut self) -> MinflW<'_, PacketlenSpec> {
        MinflW::new(self, 16)
    }
}
#[doc = "Packet Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`packetlen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`packetlen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PacketlenSpec;
impl crate::RegisterSpec for PacketlenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`packetlen::R`](R) reader structure"]
impl crate::Readable for PacketlenSpec {}
#[doc = "`write(|w| ..)` method takes [`packetlen::W`](W) writer structure"]
impl crate::Writable for PacketlenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PACKETLEN to value 0x0040_0600"]
impl crate::Resettable for PacketlenSpec {
    const RESET_VALUE: u32 = 0x0040_0600;
}
