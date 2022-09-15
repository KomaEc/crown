use rustc_middle::ty::Ty;

pub type Measure = u32;

pub trait Measurable {
    #[inline]
    fn contains_ptr(&self, ty: Ty) -> bool {
        self.measure(ty) > 0
    }
    fn measure(&self, ty: Ty) -> Measure;
}
