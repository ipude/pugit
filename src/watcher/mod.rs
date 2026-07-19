#[allow(dead_code)]
enum Signal {
  Head,
  Index,
  RefHeads,
  RefTags,
  PackedRef,
}

#[allow(dead_code)]
struct BooleanInterpreter {
  head_chng: bool,
  index_chng: bool,
  refheads_chng: bool,
  reftags_chng: bool,
  packedref_chng: bool,
}


