
#[macro_export]
macro_rules! setbit{
    ($num:expr,$b:expr)=>{
        $num | (1 << ($b as u8))
    };
    ($num:expr,$b:expr,$t:ty)=>{
        ($num as $t) | ((1 as $t) << ($b as $t))
    };
}

#[macro_export]
macro_rules! clearbit{
    ($num:expr,$b:expr)=>{
        $num & !(1 << ($b as u8))
    };
    ($num:expr,$b:expr,$t:ty)=>{
        ($num as $t) & ~((1 as $t) << ($b  as $t))
    };
}

#[macro_export]
macro_rules! bitset{
    ($num:expr,$b:expr)=>{
        ($num >> ($b as u8) & 1)
    };
}
