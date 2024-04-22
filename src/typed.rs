mod untyped;

use untyped::*;

enum Bool {
    True,
    False,
}

struct Nat {
    untyped:Term::Zero,
}
