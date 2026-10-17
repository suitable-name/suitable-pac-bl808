#[doc = "Register `MAC_ADDR1` reader"]
pub type R = crate::R<MacAddr1Spec>;
#[doc = "Register `MAC_ADDR1` writer"]
pub type W = crate::W<MacAddr1Spec>;
#[doc = "Field `MAC_B1` reader - "]
pub type MacB1R = crate::FieldReader;
#[doc = "Field `MAC_B1` writer - "]
pub type MacB1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MAC_B0` reader - "]
pub type MacB0R = crate::FieldReader;
#[doc = "Field `MAC_B0` writer - "]
pub type MacB0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn mac_b1(&self) -> MacB1R {
        MacB1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn mac_b0(&self) -> MacB0R {
        MacB0R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn mac_b1(&mut self) -> MacB1W<'_, MacAddr1Spec> {
        MacB1W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn mac_b0(&mut self) -> MacB0W<'_, MacAddr1Spec> {
        MacB0W::new(self, 8)
    }
}
#[doc = "MAC Individual Address Register 1 (Bytes 1-0)\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_addr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_addr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacAddr1Spec;
impl crate::RegisterSpec for MacAddr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_addr1::R`](R) reader structure"]
impl crate::Readable for MacAddr1Spec {}
#[doc = "`write(|w| ..)` method takes [`mac_addr1::W`](W) writer structure"]
impl crate::Writable for MacAddr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC_ADDR1 to value 0"]
impl crate::Resettable for MacAddr1Spec {}
