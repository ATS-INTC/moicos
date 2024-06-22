#[doc = "Register `remove` reader"]
pub type R = crate::R<RemoveSpec>;
#[doc = "Register `remove` writer"]
pub type W = crate::W<RemoveSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RemoveSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "Remove the specific task.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remove::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remove::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RemoveSpec;
impl crate::RegisterSpec for RemoveSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`remove::R`](R) reader structure"]
impl crate::Readable for RemoveSpec {}
#[doc = "`write(|w| ..)` method takes [`remove::W`](W) writer structure"]
impl crate::Writable for RemoveSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets remove to value 0"]
impl crate::Resettable for RemoveSpec {
    const RESET_VALUE: u64 = 0;
}
