#[doc = "Register `Top_Config` reader"]
pub type R = crate::R<TopConfigSpec>;
#[doc = "Register `Top_Config` writer"]
pub type W = crate::W<TopConfigSpec>;
#[doc = "Field `E` reader - DMA Controller Enable"]
pub type ER = crate::BitReader;
#[doc = "Field `E` writer - DMA Controller Enable"]
pub type EW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M` reader - AHB Master endianness: 0=Little, 1=Big"]
pub type MR = crate::BitReader;
#[doc = "Field `M` writer - AHB Master endianness: 0=Little, 1=Big"]
pub type MW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA Controller Enable"]
    #[inline(always)]
    pub fn e(&self) -> ER {
        ER::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AHB Master endianness: 0=Little, 1=Big"]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Controller Enable"]
    #[inline(always)]
    pub fn e(&mut self) -> EW<'_, TopConfigSpec> {
        EW::new(self, 0)
    }
    #[doc = "Bit 1 - AHB Master endianness: 0=Little, 1=Big"]
    #[inline(always)]
    pub fn m(&mut self) -> MW<'_, TopConfigSpec> {
        MW::new(self, 1)
    }
}
#[doc = "DMA Top Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`top_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`top_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TopConfigSpec;
impl crate::RegisterSpec for TopConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`top_config::R`](R) reader structure"]
impl crate::Readable for TopConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`top_config::W`](W) writer structure"]
impl crate::Writable for TopConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Top_Config to value 0"]
impl crate::Resettable for TopConfigSpec {}
