pub mod old {
    pub trait Abc {
        type A;
        fn test(&self) -> u8;
        fn test2(&self) -> u8;
        fn test4() -> u8 {
            0
        }
        fn test5() -> u8 {
            0
        }
        fn test7() -> u8;
        fn test8(&self) -> u8;
        fn test9(_: &Self) -> u8;
    }
}

pub mod new {
    pub trait Abc {
        type A;
        fn test(&self) -> u8;
        fn test3(&self) -> u8;
        fn test4() -> u8 {
            0
        }
        fn test6() -> u8 {
            0
        }
        fn test7() -> u16;
        fn test8(_: &Self) -> u8;
        fn test9(&self) -> u8;
    }
}