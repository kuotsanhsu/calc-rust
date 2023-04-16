mod universe;
use universe::Universe;

struct Context;
trait Term<Type: Term<Universe>> {
    fn r#type(&self) -> Type;
}

// mod test {
//     struct Universe(usize);
//     trait Term {}
//     struct Judgement<Type: Term> {
//         term: dyn Term,
//         r#type: Type,
//     }
// }
