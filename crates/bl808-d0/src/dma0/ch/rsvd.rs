#[doc = "Register `RSVD` reader"]
pub type R = crate::R<RsvdSpec>;
#[doc = "Register `RSVD` writer"]
pub type W = crate::W<RsvdSpec>;
#[doc = "Field `DstRemnSgle` reader - "]
pub type DstRemnSgleR = crate::BitReader;
#[doc = "Field `DstRemnSgle` writer - "]
pub type DstRemnSgleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SrcRemnSgle` reader - "]
pub type SrcRemnSgleR = crate::BitReader;
#[doc = "Field `SrcRemnSgle` writer - "]
pub type SrcRemnSgleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dst_remn_sgle(&self) -> DstRemnSgleR {
        DstRemnSgleR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn src_remn_sgle(&self) -> SrcRemnSgleR {
        SrcRemnSgleR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dst_remn_sgle(&mut self) -> DstRemnSgleW<'_, RsvdSpec> {
        DstRemnSgleW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn src_remn_sgle(&mut self) -> SrcRemnSgleW<'_, RsvdSpec> {
        SrcRemnSgleW::new(self, 4)
    }
}
#[doc = "Channel Reserved Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsvd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsvd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RsvdSpec;
impl crate::RegisterSpec for RsvdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsvd::R`](R) reader structure"]
impl crate::Readable for RsvdSpec {}
#[doc = "`write(|w| ..)` method takes [`rsvd::W`](W) writer structure"]
impl crate::Writable for RsvdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RSVD to value 0"]
impl crate::Resettable for RsvdSpec {}
