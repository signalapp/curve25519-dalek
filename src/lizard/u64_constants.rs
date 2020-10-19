use backend::serial::u64::field::FieldElement51;

/// `= sqrt(i*d)`, where `i = +sqrt(-1)` and `d` is the Edwards curve parameter.
pub const SQRT_ID: FieldElement51 = FieldElement51([2298852427963285, 3837146560810661, 4413131899466403, 3883177008057528, 2352084440532925]);

/// `= (d+1)/(d-1)`, where `d` is the Edwards curve parameter.
pub const DP1_OVER_DM1: FieldElement51 = FieldElement51([2159851467815724, 1752228607624431, 1825604053920671, 1212587319275468, 253422448836237]);

/// `= -2/sqrt(a-d)`, where `a = -1 (mod p)`, `d` are the Edwards curve parameters.
pub const MDOUBLE_INVSQRT_A_MINUS_D: FieldElement51 = FieldElement51([1693982333959686, 608509411481997, 2235573344831311, 947681270984193, 266558006233600]);

/// `= -2i/sqrt(a-d)`, where `a = -1 (mod p)`, `d` are the Edwards curve parameters
/// and `i = +sqrt(-1)`.
pub const MIDOUBLE_INVSQRT_A_MINUS_D: FieldElement51 = FieldElement51([1608655899704280, 1999971613377227, 49908634785720, 1873700692181652, 353702208628067]);

/// `= -1/sqrt(1+d)`, where `d` is the Edwards curve parameters.
pub const MINVSQRT_ONE_PLUS_D: FieldElement51 = FieldElement51([321571956990465, 1251814006996634, 2226845496292387, 189049560751797, 2074948709371214]);

