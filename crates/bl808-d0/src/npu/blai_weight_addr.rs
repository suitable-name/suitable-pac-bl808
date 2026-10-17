#[doc = "Register `BLAI_WEIGHT_ADDR` reader"]
pub type R = crate::R<BlaiWeightAddrSpec>;
#[doc = "Register `BLAI_WEIGHT_ADDR` writer"]
pub type W = crate::W<BlaiWeightAddrSpec>;
#[doc = "Field `reg_weig_addr_base` reader - Base address for weight data."]
pub type RegWeigAddrBaseR = crate::FieldReader<u32>;
#[doc = "Field `reg_weig_addr_base` writer - Base address for weight data."]
pub type RegWeigAddrBaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Base address for weight data."]
    #[inline(always)]
    pub fn reg_weig_addr_base(&self) -> RegWeigAddrBaseR {
        RegWeigAddrBaseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address for weight data."]
    #[inline(always)]
    pub fn reg_weig_addr_base(&mut self) -> RegWeigAddrBaseW<'_, BlaiWeightAddrSpec> {
        RegWeigAddrBaseW::new(self, 0)
    }
}
#[doc = "Weight Data Base Address Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_weight_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_weight_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiWeightAddrSpec;
impl crate::RegisterSpec for BlaiWeightAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_weight_addr::R`](R) reader structure"]
impl crate::Readable for BlaiWeightAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`blai_weight_addr::W`](W) writer structure"]
impl crate::Writable for BlaiWeightAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_WEIGHT_ADDR to value 0"]
impl crate::Resettable for BlaiWeightAddrSpec {}
