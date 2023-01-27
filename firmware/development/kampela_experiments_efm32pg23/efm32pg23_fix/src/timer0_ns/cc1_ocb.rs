#[doc = "Register `CC1_OCB` reader"]
pub struct R(crate::R<CC1_OCB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC1_OCB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC1_OCB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC1_OCB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC1_OCB` writer"]
pub struct W(crate::W<CC1_OCB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC1_OCB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CC1_OCB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC1_OCB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCB` reader - Output Compare Value Buffer"]
pub type OCB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OCB` writer - Output Compare Value Buffer"]
pub type OCB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CC1_OCB_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Output Compare Value Buffer"]
    #[inline(always)]
    pub fn ocb(&self) -> OCB_R {
        OCB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Output Compare Value Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn ocb(&mut self) -> OCB_W<0> {
        OCB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc1_ocb](index.html) module"]
pub struct CC1_OCB_SPEC;
impl crate::RegisterSpec for CC1_OCB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc1_ocb::R](R) reader structure"]
impl crate::Readable for CC1_OCB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc1_ocb::W](W) writer structure"]
impl crate::Writable for CC1_OCB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CC1_OCB to value 0"]
impl crate::Resettable for CC1_OCB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}