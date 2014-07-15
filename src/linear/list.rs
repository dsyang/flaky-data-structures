

#[deriving(PartialEq, Clone, Show)]
pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>)
}

pub fn cons<T:Clone> (data : T, seq : Box<List<T>>) -> Box<List<T>> {
    box Cons(data, seq)
}

pub fn nil<T>() -> Box<List<T>> {
    box Nil
}

// Convert from a vector to a linked list
pub fn from_vec<T:Clone> (v : &[T]) -> Box<List<T>> {
    let mut accum = nil();
    let len : uint = v.len();
    let mut i = len - 1;

    while i < len {
        accum = cons(v[i].clone(), accum);
        i -= 1;
    }

    accum
}

pub fn first<T:Clone> (list : &Box<List<T>>) -> Option<T> {
    match list {
        &box Nil => { None }
        &box Cons(ref e, _) =>  {
            Some(e.clone())
        }
    }
}

pub fn rest<T:Clone> (list : &Box<List<T>>) -> Box<List<T>> {
    match list {
        &box Nil => {
            nil()
        }
        &box Cons(_, ref rest) => {
            rest.clone()
        }
    }
}


#[quickcheck]
fn qc_from_vec(xs: Vec<int>) -> bool {
    let l = from_vec(xs.as_slice());

    let mut list = l;
    let mut it = xs.iter();
    let mut elt = it.next();
    loop {
        match (*list, elt) {
            (Nil, None) => { return true }
            (Cons(x, xs), Some(&i)) => {
                if x != i { return false }
                list = xs;
                elt = it.next();
            }
            _ => { return false }
        }
    }
}

#[quickcheck]
fn qc_first(xs: Vec<int>) -> bool {
    let l = from_vec(xs.as_slice());
    match (first(&l)) {
        None => { *l == Nil }
        Some(e) => {
            match *l {
                Nil => {false}
                Cons(x, _ ) =>{ e == x}
            }
        }
    }
}

#[quickcheck]
fn qc_rest(xs: Vec<int>) -> bool {
    let l = from_vec(xs.as_slice());
    match *rest(&l) {
        Nil => { xs.len() == 0 || xs.len() == 1}
        other => {
            match *l {
                Cons (_, box ls) => {
                    other == ls
                }
                Nil => { false }
            }
        }
    }
}
