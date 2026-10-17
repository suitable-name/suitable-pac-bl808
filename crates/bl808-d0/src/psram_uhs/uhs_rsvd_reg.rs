#[doc = "Register `UHS_RSVD_REG` reader"]
pub type R = crate::R<UhsRsvdRegSpec>;
#[doc = "Register `UHS_RSVD_REG` writer"]
pub type W = crate::W<UhsRsvdRegSpec>;
#[doc = "Field `reg_mr0_7` reader - MR0 bit 7"]
pub type RegMr0_7R = crate::BitReader;
#[doc = "Field `reg_mr0_7` writer - MR0 bit 7"]
pub type RegMr0_7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mr2_2_0` reader - MR2 bits 2-0"]
pub type RegMr2_2_0R = crate::FieldReader;
#[doc = "Field `reg_mr2_2_0` writer - MR2 bits 2-0"]
pub type RegMr2_2_0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_mr2_7_6` reader - MR2 bits 7-6"]
pub type RegMr2_7_6R = crate::FieldReader;
#[doc = "Field `reg_mr2_7_6` writer - MR2 bits 7-6"]
pub type RegMr2_7_6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - MR0 bit 7"]
    #[inline(always)]
    pub fn reg_mr0_7(&self) -> RegMr0_7R {
        RegMr0_7R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - MR2 bits 2-0"]
    #[inline(always)]
    pub fn reg_mr2_2_0(&self) -> RegMr2_2_0R {
        RegMr2_2_0R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5 - MR2 bits 7-6"]
    #[inline(always)]
    pub fn reg_mr2_7_6(&self) -> RegMr2_7_6R {
        RegMr2_7_6R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MR0 bit 7"]
    #[inline(always)]
    pub fn reg_mr0_7(&mut self) -> RegMr0_7W<'_, UhsRsvdRegSpec> {
        RegMr0_7W::new(self, 0)
    }
    #[doc = "Bits 1:3 - MR2 bits 2-0"]
    #[inline(always)]
    pub fn reg_mr2_2_0(&mut self) -> RegMr2_2_0W<'_, UhsRsvdRegSpec> {
        RegMr2_2_0W::new(self, 1)
    }
    #[doc = "Bits 4:5 - MR2 bits 7-6"]
    #[inline(always)]
    pub fn reg_mr2_7_6(&mut self) -> RegMr2_7_6W<'_, UhsRsvdRegSpec> {
        RegMr2_7_6W::new(self, 4)
    }
}
#[doc = "UHS Mode Register Reserved Bits\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_rsvd_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_rsvd_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsRsvdRegSpec;
impl crate::RegisterSpec for UhsRsvdRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_rsvd_reg::R`](R) reader structure"]
impl crate::Readable for UhsRsvdRegSpec {}
#[doc = "`write(|w| ..)` method takes [`uhs_rsvd_reg::W`](W) writer structure"]
impl crate::Writable for UhsRsvdRegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHS_RSVD_REG to value 0"]
impl crate::Resettable for UhsRsvdRegSpec {}
