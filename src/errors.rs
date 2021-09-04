#[derive(Debug, PartialEq)]
pub enum CustomErrorKind {
    CoordinateIsOutsideOfPosition,
}

// TODO: Err(error) でエラー種別の判定をしたくて作った。一般的に Rust でどうするのか不明。
#[derive(Debug)]
pub struct CustomError {
    pub kind: CustomErrorKind,
}
