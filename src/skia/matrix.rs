use std::mem;
use rust_skia::SkMatrix;
use rust_skia::SkMatrix_TypeMask;
use crate::prelude::*;
use crate::skia::MatrixTypeMask;
use crate::skia::Scalar;
use std::ops::Index;
use std::ops::IndexMut;
use crate::skia::Vector;
use crate::skia::Point;
use rust_skia::SkMatrix_ScaleToFit;
use crate::skia::Rect;
use rust_skia::SkPoint;

pub type MatrixScaleToFit = EnumHandle<SkMatrix_ScaleToFit>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkMatrix_ScaleToFit> {
    pub const Fill: Self = Self(SkMatrix_ScaleToFit::kFill_ScaleToFit);
    pub const Start: Self = Self(SkMatrix_ScaleToFit::kStart_ScaleToFit);
    pub const Center: Self = Self(SkMatrix_ScaleToFit::kCenter_ScaleToFit);
    pub const End: Self = Self(SkMatrix_ScaleToFit::kEnd_ScaleToFit);
}

pub type Matrix = ValueHandle<SkMatrix>;

impl NativePartialEq for SkMatrix {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { rust_skia::C_SkMatrix_Equals(self, rhs) }
    }
}

pub enum MatrixMember {
    ScaleX = 0,
    SkewX = 1,
    TransX = 2,
    SkewY = 3,
    ScaleY = 4,
    TransY = 5,
    Persp0 = 6,
    Persp1 = 7,
    Persp2 = 8
}

pub enum AffineMatrixMember {
    ScaleX = 0,
    SkewY = 1,
    SkewX = 2,
    ScaleY = 3,
    TransX = 4,
    TransY = 5
}

impl Index<MatrixMember> for Matrix {
    type Output = f32;

    fn index(&self, index: MatrixMember) -> &Self::Output {
        &self[index as usize]
    }
}

impl Index<AffineMatrixMember> for Matrix {
    type Output = f32;

    fn index(&self, index: AffineMatrixMember) -> &Self::Output {
        &self[index as usize]
    }
}

impl Index<usize> for Matrix {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.native().fMat[index]
    }
}

impl IndexMut<MatrixMember> for Matrix {
    fn index_mut(&mut self, index: MatrixMember) -> &mut Self::Output {
        self.index_mut(index as usize)
    }
}

impl IndexMut<AffineMatrixMember> for Matrix {
    fn index_mut(&mut self, index: AffineMatrixMember) -> &mut Self::Output {
        self.index_mut(index as usize)
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.native_mut().fMat.index_mut(index)
    }
}

impl Matrix {

    pub fn new_scale(sx: f32, sy: f32) -> Matrix {
        unsafe { SkMatrix::MakeScale(sx, sy) }.into_handle()
    }

    pub fn new_trans(dx: f32, dy: f32) -> Matrix {
        unsafe { SkMatrix::MakeTrans(dx, dy) }.into_handle()
    }

    pub fn new_all(
        scale_x: f32, skew_x: f32, trans_x: f32,
        skew_y: f32, scale_y: f32, trans_y: f32,
        pers_0: f32, pers_1: f32, pers_2: f32) -> Matrix {
        unsafe { SkMatrix::MakeAll(
            scale_x, skew_x, trans_x,
            skew_y, scale_y, trans_y,
            pers_0, pers_1, pers_2)
        }.into_handle()
    }

    pub fn get_type(&self) -> MatrixTypeMask {
        unsafe { self.native().getType() }.into()
    }

    pub fn is_identity(&self) -> bool {
        unsafe { self.native().isIdentity() }
    }

    pub fn is_scale_translate(&self) -> bool {
        unsafe { self.native().isScaleTranslate() }
    }

    pub fn is_translate(&self) -> bool {
        // isTranslate does not link
        (self.get_type() & !MatrixTypeMask::Translate).is_empty()
    }

    pub fn rect_stays_rect(&self) -> bool {
        unsafe { self.native().rectStaysRect() }
    }

    pub fn preserves_axis_alignment(&self) -> bool {
        unsafe { self.native().preservesAxisAlignment() }
    }

    pub fn has_perspective(&self) -> bool {
        unsafe { self.native().hasPerspective() }
    }

    pub fn is_similarity(&self) -> bool {
        unsafe { self.native().isSimilarity(f32::NEARLY_ZERO) }
    }

    pub fn preserves_right_angles(&self) -> bool {
        unsafe { self.native().preservesRightAngles(f32::NEARLY_ZERO) }
    }

    pub fn get_scale_x(&self) -> f32 {
        unsafe { self.native().getScaleX() }
    }

    pub fn get_scale_y(&self) -> f32 {
        unsafe { self.native().getScaleY() }
    }

    pub fn get_skew_y(&self) -> f32 {
        unsafe { self.native().getSkewY() }
    }

    pub fn get_skew_x(&self) -> f32 {
        unsafe { self.native().getSkewX() }
    }

    pub fn get_translate_x(&self) -> f32 {
        unsafe { self.native().getTranslateX() }
    }

    pub fn get_translate_y(&self) -> f32 {
        unsafe { self.native().getTranslateY() }
    }

    pub fn get_persp_x(&self) -> f32 {
        unsafe { self.native().getPerspX() }
    }

    pub fn get_persp_y(&self) -> f32 {
        unsafe { self.native().getPerspY() }
    }

    pub fn set_scale_x(&mut self, v: f32) {
        self[MatrixMember::ScaleX] = v;
    }

    pub fn set_scale_y(&mut self, v: f32) {
        self[MatrixMember::ScaleY] = v;
    }

    pub fn set_skew_y(&mut self, v: f32) {
        self[MatrixMember::SkewY] = v;
    }

    pub fn set_skew_x(&mut self, v: f32) {
        self[MatrixMember::SkewX] = v;
    }

    pub fn set_translate_x(&mut self, v: f32) {
        self[MatrixMember::TransX] = v;
    }

    pub fn set_translate_y(&mut self, v: f32) {
        self[MatrixMember::TransY] = v;
    }

    pub fn set_persp_x(&mut self, v: f32) {
        self[MatrixMember::Persp0] = v;
    }

    pub fn set_persp_y(&mut self, v: f32) {
        self[MatrixMember::Persp1] = v;
    }

    pub fn get_9(&self, buffer: &mut[f32; 9]) {
        unsafe { self.native().get9(buffer.as_mut_ptr()) }
    }

    pub fn set_9(&mut self, buffer: &[f32; 9]) {
        unsafe { self.native_mut().set9(buffer.as_ptr()) }
    }

    pub fn reset(&mut self) {
        unsafe { self.native_mut().reset() }
    }

    pub fn set_identity(&mut self) {
        unsafe { self.native_mut().setIdentity() }
    }

    pub fn set_translate(&mut self, v: Vector) {
        unsafe { self.native_mut().setTranslate(v.x, v.y) }
    }

    pub fn set_scale(&mut self, sx: f32, sy: f32, pivot: Option<Point>) {
        let pivot = pivot.unwrap_or(Point::new(0.0, 0.0));
        unsafe { self.native_mut().setScale(sx, sy, pivot.x, pivot.y) }
    }

    pub fn set_rotate(&mut self, degrees: f32, pivot: Option<Point>) {
        let pivot = pivot.unwrap_or(Point::new(0.0, 0.0));
        unsafe { self.native_mut().setRotate(degrees, pivot.x, pivot.y) }
    }

    pub fn set_sin_cos(&mut self, sin_value: f32, cos_value: f32, pivot: Option<Point>) {
        let pivot = pivot.unwrap_or(Point::new(0.0, 0.0));
        unsafe { self.native_mut().setSinCos(sin_value, cos_value, pivot.x, pivot.y) }
    }

    pub fn set_skew(&mut self, kx: f32, ky: f32, pivot: Option<Point>) {
        let pivot = pivot.unwrap_or(Point::new(0.0, 0.0));
        unsafe { self.native_mut().setSkew(kx, ky, pivot.x, pivot.y) }
    }

    pub fn set_concat(&mut self, a: &Matrix, b: &Matrix) {
        unsafe { self.native_mut().setConcat(a.native(), b.native()) }
    }

    pub fn pre_translate(&mut self, delta: Vector) {
        unsafe { self.native_mut().preTranslate(delta.x, delta.y) }
    }

    pub fn pre_scale(&mut self, sx: f32, sy: f32, pivot: Option<Point>) {
        let pivot = pivot.unwrap_or(Point::new(0.0, 0.0));
        unsafe { self.native_mut().preScale(sx, sy, pivot.x, pivot.y) }
    }

    pub fn pre_rotate(&mut self, degrees: f32, pivot: Option<Point>) {
        let pivot = pivot.unwrap_or(Point::new(0.0, 0.0));
        unsafe { self.native_mut().preRotate(degrees, pivot.x, pivot.y) }
    }

    pub fn pre_skew(&mut self, kx: f32, ky: f32, pivot: Option<Point>) {
        let pivot = pivot.unwrap_or(Point::new(0.0, 0.0));
        unsafe { self.native_mut().preSkew(kx, ky, pivot.x, pivot.y) }
    }

    pub fn pre_concat(&mut self, other: &Matrix) {
        unsafe { self.native_mut().preConcat(other.native()) }
    }

    pub fn post_translate(&mut self, delta: Vector) {
        unsafe { self.native_mut().postTranslate(delta.x, delta.y) }
    }

    pub fn post_scale(&mut self, sx: f32, sy: f32, pivot: Option<Point>) {
        let pivot = pivot.unwrap_or(Point::new(0.0, 0.0));
        unsafe { self.native_mut().postScale(sx, sy, pivot.x, pivot.y) }
    }

    pub fn post_idiv(&mut self, div_x: i32, div_y: i32) -> bool {
        unsafe { self.native_mut().postIDiv(div_x, div_y) }
    }

    pub fn post_rotate(&mut self, degrees: f32, pivot: Option<Point>) {
        let pivot = pivot.unwrap_or(Point::new(0.0, 0.0));
        unsafe { self.native_mut().postRotate(degrees, pivot.x, pivot.y) }
    }

    pub fn post_skew(&mut self, kx: f32, ky: f32, pivot: Option<Point>) {
        let pivot = pivot.unwrap_or(Point::new(0.0, 0.0));
        unsafe { self.native_mut().postSkew(kx, ky, pivot.x, pivot.y) }
    }

    pub fn post_concat(&mut self, other: &Matrix) {
        unsafe { self.native_mut().postConcat(other.native()) }
    }

    pub fn new_rect_to_rect(src: &Rect, dst: &Rect, stf: MatrixScaleToFit) -> Option<Matrix> {
        let mut m = Matrix::new_identity();
        if unsafe { m.native_mut().setRectToRect(&src.to_native(), &dst.to_native(), stf.native().to_owned()) } {
            Some(m)
        } else {
            None
        }
    }

    pub fn new_poly_to_poly(src: &[Point], dst: &[Point]) -> Option<Matrix> {
        if src.len() != dst.len() {
            return None
        }

        let src : Vec<SkPoint> = src.iter().map(|p| p.to_native()).collect();
        let dst : Vec<SkPoint> = dst.iter().map(|p| p.to_native()).collect();

        let mut m = Matrix::new_identity();
        if unsafe { m.native_mut().setPolyToPoly(src.as_ptr(), dst.as_ptr(), src.len() as _) } {
            Some(m)
        } else {
            None
        }
    }

    #[warn(unused)]
    pub fn invert(&self) -> Option<Matrix> {
        let mut m = Matrix::new_identity();
        if unsafe { self.native().invert(m.native_mut()) } {
            Some(m)
        } else {
            None
        }
    }

    pub fn set_affine_identity(affine: &mut [f32; 6]) {
        unsafe { SkMatrix::SetAffineIdentity(affine.as_mut_ptr()) }
    }

    #[warn(unused)]
    pub fn as_affine(&mut self) -> Option<[f32; 6]> {
        let mut affine = [0.0; 6];
        if unsafe { self.native_mut().asAffine(affine.as_mut_ptr()) } {
            Some(affine)
        } else {
            None
        }
    }

    pub fn new_affine(affine: &[f32; 6]) -> Matrix {
        let mut m = Matrix::new_identity();
        unsafe { m.native_mut().setAffine(affine.as_ptr()) }
        m
    }

    




    pub fn new_identity() -> Matrix {
        // SkMatrix contains no C++ types, so this is safe:
        let mut m : SkMatrix = unsafe { mem::zeroed() };
        unsafe { m.reset() };
        Matrix::from_native(m)
    }
}

impl IndexGet for Matrix {}
impl IndexSet for Matrix {}

#[test]
fn test_get_set_trait_compilation() {
    let mut m = Matrix::new_identity();
    let x = m.get(AffineMatrixMember::ScaleX);
    m.set(AffineMatrixMember::ScaleX, 1.0);
}

fn test_tuple_to_vector() {
    let mut m = Matrix::new_identity();
    m.set_translate((10.0, 10.0).lift())
}