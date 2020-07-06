pub const NOP: i32 = 0; // do nothing
    // *n is dereference operation
    // add a b c - vm.mem[c] = a + b
pub const ADD_VVA: i32 = 1; // add a b c
pub const ADD_AAA: i32 = 2; // add *a *b c
pub const ADD_AVA: i32 = 3; // add *a b c
pub const ADD_DDA: i32 = 4; // add **a **b c
pub const ADD_DAA: i32 = 5; // add **a *b c
pub const ADD_DVA: i32 = 6; // add **a b c
pub const ADD_VVD: i32 = 7; // add a b *c
pub const ADD_AAD: i32 = 8; // add *a *b *c
pub const ADD_AVD: i32 = 9; // add *a b *c
pub const ADD_DDD: i32 = 10; // add **a **b *c
pub const ADD_DAD: i32 = 11; // add **a *b *c
pub const ADD_DVD: i32 = 12; // add **a b *c