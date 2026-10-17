#[doc = "Register `BLAI_INST_ADDR` reader"]
pub type R = crate::R<BlaiInstAddrSpec>;
#[doc = "Register `BLAI_INST_ADDR` writer"]
pub type W = crate::W<BlaiInstAddrSpec>;
#[doc = "Field `reg_inst_addr_base` reader - Base address for NPU instructions."]
pub type RegInstAddrBaseR = crate::FieldReader<u32>;
#[doc = "Field `reg_inst_addr_base` writer - Base address for NPU instructions."]
pub type RegInstAddrBaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Base address for NPU instructions."]
    #[inline(always)]
    pub fn reg_inst_addr_base(&self) -> RegInstAddrBaseR {
        RegInstAddrBaseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address for NPU instructions."]
    #[inline(always)]
    pub fn reg_inst_addr_base(&mut self) -> RegInstAddrBaseW<'_, BlaiInstAddrSpec> {
        RegInstAddrBaseW::new(self, 0)
    }
}
#[doc = "Instruction Command Base Address Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_inst_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_inst_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiInstAddrSpec;
impl crate::RegisterSpec for BlaiInstAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_inst_addr::R`](R) reader structure"]
impl crate::Readable for BlaiInstAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`blai_inst_addr::W`](W) writer structure"]
impl crate::Writable for BlaiInstAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_INST_ADDR to value 0"]
impl crate::Resettable for BlaiInstAddrSpec {}
