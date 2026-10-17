#[doc = "Register `AON` reader"]
pub type R = crate::R<AonSpec>;
#[doc = "Register `AON` writer"]
pub type W = crate::W<AonSpec>;
#[doc = "Field `aon_resv` reader - "]
pub type AonResvR = crate::FieldReader;
#[doc = "Field `aon_resv` writer - "]
pub type AonResvW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pu_aon_dc_tbuf` reader - "]
pub type PuAonDcTbufR = crate::BitReader;
#[doc = "Field `pu_aon_dc_tbuf` writer - "]
pub type PuAonDcTbufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo11_rt_pulldown` reader - "]
pub type Ldo11RtPulldownR = crate::BitReader;
#[doc = "Field `ldo11_rt_pulldown` writer - "]
pub type Ldo11RtPulldownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo11_rt_pulldown_sel` reader - "]
pub type Ldo11RtPulldownSelR = crate::BitReader;
#[doc = "Field `ldo11_rt_pulldown_sel` writer - "]
pub type Ldo11RtPulldownSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sw_pu_ldo11_rt` reader - "]
pub type SwPuLdo11RtR = crate::BitReader;
#[doc = "Field `sw_pu_ldo11_rt` writer - "]
pub type SwPuLdo11RtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn aon_resv(&self) -> AonResvR {
        AonResvR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_aon_dc_tbuf(&self) -> PuAonDcTbufR {
        PuAonDcTbufR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ldo11_rt_pulldown(&self) -> Ldo11RtPulldownR {
        Ldo11RtPulldownR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ldo11_rt_pulldown_sel(&self) -> Ldo11RtPulldownSelR {
        Ldo11RtPulldownSelR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sw_pu_ldo11_rt(&self) -> SwPuLdo11RtR {
        SwPuLdo11RtR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn aon_resv(&mut self) -> AonResvW<'_, AonSpec> {
        AonResvW::new(self, 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_aon_dc_tbuf(&mut self) -> PuAonDcTbufW<'_, AonSpec> {
        PuAonDcTbufW::new(self, 12)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ldo11_rt_pulldown(&mut self) -> Ldo11RtPulldownW<'_, AonSpec> {
        Ldo11RtPulldownW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ldo11_rt_pulldown_sel(&mut self) -> Ldo11RtPulldownSelW<'_, AonSpec> {
        Ldo11RtPulldownSelW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sw_pu_ldo11_rt(&mut self) -> SwPuLdo11RtW<'_, AonSpec> {
        SwPuLdo11RtW::new(self, 22)
    }
}
#[doc = "AON Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonSpec;
impl crate::RegisterSpec for AonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon::R`](R) reader structure"]
impl crate::Readable for AonSpec {}
#[doc = "`write(|w| ..)` method takes [`aon::W`](W) writer structure"]
impl crate::Writable for AonSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AON to value 0"]
impl crate::Resettable for AonSpec {}
