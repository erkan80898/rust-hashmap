mod hash_map;
use hash_map::HashMap;

fn main() {

    let mut hashmap = HashMap::<char,u32>::new();

    hashmap.put('a',1);
    hashmap.put('b',2);
    hashmap.put('c',3);

    for (&x,&y) in &hashmap{
        match x{
            'a' => assert_eq!(y,1),
            'b' => assert_eq!(y,2),
            'c' => assert_eq!(y,3),
            _ => unreachable!()
        }
    }
}
