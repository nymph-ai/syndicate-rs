/*
Try

(echo '[
  [0 <A #:[0 0] 1>]
  [0 <A 123.45 2>]
  [0 <A 99999 3>]
  [0 <A 9 4>]
  [0 <A -9 5>]
  [0 <A "hi" 6>]
  [0 <A #"hi" 7>]
  [0 <A hi 8>]
  [0 <A [1 2 3] 9>]
  [0 <A [1 2000 3] 10>]
  [0 <A {a: -6} 11>]
  [0 <A {a: 6} 12>]
  [0 <A {x: 7} 13>]
  [0 <A <x y z> 14>]
  [0 <A <q y z> 15>]
]'; sleep 2) | cargo run --example stdio-patterns

*/

use preserves::*;

fn show<T: std::fmt::Debug + Sync + Send + Clone + 'static>(
    t: &mut Activation,
    label: &'static str,
    v: T
) -> ActorResult {
    t.on_stop(enclose!((v) move |_t| { tracing::info!("- {label}: {v:?}"); Ok(()) }));
    tracing::info!("+ {label}: {v:?}");
    Ok(())
}

syndicate::stdio_service!{
    ($v) => |t| show(t, "--------------------", v);
    ($v: AnyValue) => |t| show(t, "AnyValue", v);
    ($v: Embedded<Arc<Cap>>) => |t| show(t, "Embedded<Arc<Cap>>", v);
    ($v: Arc<Cap>) => |t| show(t, "Arc<Cap>", v);
    ($v: Double) => |t| show(t, "Double", v);
    ($v: f64) => |t| show(t, "f64", v);
    ($v: SignedInteger) => |t| show(t, "SignedInteger", v);
    ($v: u8) => |t| show(t, "u8", v);
    ($v: i8) => |t| show(t, "i8", v);
    ($v: u16) => |t| show(t, "u16", v);
    ($v: i16) => |t| show(t, "i16", v);
    ($v: u32) => |t| show(t, "u32", v);
    ($v: i32) => |t| show(t, "i32", v);
    ($v: u64) => |t| show(t, "u64", v);
    ($v: i64) => |t| show(t, "i64", v);
    ($v: u128) => |t| show(t, "u128", v);
    ($v: i128) => |t| show(t, "i128", v);
    ($v: usize) => |t| show(t, "usize", v);
    ($v: isize) => |t| show(t, "isize", v);
    ($v: String) => |t| show(t, "String", v);
    ($v: Bytes) => |t| show(t, "Bytes", v);
    ($v: Symbol) => |t| show(t, "Symbol", v);
    ($v: Vec<u8>) => |t| show(t, "Vec<u8>", v);
    ($v: Vec<u16>) => |t| show(t, "Vec<u16>", v);
    ([1, $v: u32]) => |t| show(t, "[1, $v: u32]", v);
    ($v: Map<Symbol, i8>) => |t| show(t, "Map<Symbol, i8>", v);
    ($v: Map<Symbol, u8>) => |t| show(t, "Map<Symbol, u8>", v);
    ({a: $v: i32}) => |t| show(t, "{a: $v: i32}", v);
    (<q $y $z>) => |t| show(t, "<x $y $z>", vec![y, z]);
    (<x $y $z>) => |t| show(t, "<x $y $z>", vec![y, z]);
    (<x $y>) => |t| show(t, "<x $y>", vec![y]);
}
