#[doc = "Register `IPGT` reader"]
pub type R = crate::R<IpgtSpec>;
#[doc = "Register `IPGT` writer"]
pub type W = crate::W<IpgtSpec>;
#[doc = "Field `IPGT` reader - "]
pub type IpgtR = crate::FieldReader;
#[doc = "Field `IPGT` writer - "]
pub type IpgtW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn ipgt(&self) -> IpgtR {
        IpgtR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn ipgt(&mut self) -> IpgtW<'_, IpgtSpec> {
        IpgtW::new(self, 0)
    }
}
#[doc = "Back-to-Back Inter-Packet-Gap Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipgt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipgt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpgtSpec;
impl crate::RegisterSpec for IpgtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipgt::R`](R) reader structure"]
impl crate::Readable for IpgtSpec {}
#[doc = "`write(|w| ..)` method takes [`ipgt::W`](W) writer structure"]
impl crate::Writable for IpgtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPGT to value 0x18"]
impl crate::Resettable for IpgtSpec {
    const RESET_VALUE: u32 = 0x18;
}
