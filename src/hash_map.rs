use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;
use std::borrow::Borrow;

/// Implements a generic hashmap
///
/// The Hashmap also implements the IntoIterator trait, allowing it to be
/// used in rust's for in syntax
///
/// Separate chaining is used to deal with collisions
///
/// Initial size is 1024
///
/// When the bucket is half-way full, it will double in size

const INITIAL_SIZE:usize = 1024;

#[derive(Hash,Debug,Clone)]
pub struct HashMap<K,V>{
    buckets: Vec<Vec<(K, V)>>,
    len: usize,
}

pub struct Iter<'a,K: 'a,V: 'a>{
    map:&'a HashMap<K,V>,
    outer: usize,
    inner: usize
}

impl<'a,K,V> Iterator for Iter<'a,K,V>{
    type Item = (&'a K,&'a V);
    fn next(&mut self) -> Option<Self::Item> {
        loop{
            match self.map.buckets.get(self.outer){
                Some(inner) =>{
                    match inner.get(self.inner) {
                        Some(&(ref key,ref value)) => {
                            self.inner += 1;
                            return Some((key,value));
                        },
                        None => {
                            self.outer += 1;
                            self.inner = 0;
                            continue;
                        }
                    }
                },
                None => break None,
            }
        }
    }
}

impl<'a,K,V> IntoIterator for &'a HashMap<K,V> {
    type Item = (&'a K,&'a V);
    type IntoIter = Iter<'a,K,V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter{
            map:self,
            outer:0,
            inner:0
        }
    }
}

impl<K,V> HashMap<K,V>
where
    K: Hash + Eq + Copy,
    V: Copy
{
    ///constructs a new hashmap
    pub fn new() -> HashMap<K,V>{
        HashMap {
            buckets: Vec::new(),
            len: 0,
        }
    }

    /// Takes a reference to the key
    /// The value will be wrapped in a Some if the key has a mapping
    /// or it will return a None
    pub fn get<Q>(&self,key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized
    {

        let index = HashMap::<K,V>::calculate_hash(key,self.buckets.len());


        let val = self.buckets[index].iter().
            find(|&(ref x,_)|x.borrow() == key);

        if let Some(&(_,ref x)) = val {
            return Some(x)
        }

        None
    }

    /// Return true if hashmap has no mappings
    /// false otherwise
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Return the amount of Key -> Value mappings
    pub fn size(&self) -> usize{
        return self.len;
    }

    /// Resizes the hashmap by doubling the capacity
    fn resize(&mut self) {

        let target_size = match self.buckets.len(){
            0 => INITIAL_SIZE,
            length => 2 * length
        };


        let mut buckets = Vec::with_capacity(target_size);

        buckets.extend((0..target_size).map(|_| Vec::new()));

        //This is no good because of lazyness
        //(0..target_size).map(|_|buckets.push(Vec::new()));

        for (key,value) in self.buckets.iter_mut().flat_map(|x| x.drain(..)){

            let hash = HashMap::<K,V>::calculate_hash(&key,buckets.len());
            buckets[hash].push((key,value));
        }

        mem::replace(&mut self.buckets,buckets);
    }


    /// Inserts a key,value pair
    pub fn put(&mut self, key: K, val: V) {

        if self.buckets.len() == 0 || self.buckets.len() >= self.buckets.capacity()/2{
            self.resize();
        }

        let hash_val = HashMap::<K,V>::calculate_hash(&key,self.buckets.len());

        for (keyold,value) in self.buckets[hash_val].iter_mut(){
            if key == *keyold {
                self.len += 1;
                mem::replace(value,val);
                return;
            }
        }

        self.len += 1;
        self.buckets[hash_val].push((key, val));

    }

    /// Takes a reference to a key, and removes that key's
    /// key,value mapping.
    ///
    /// Returns the value wrapped in a Some if the key had a mapping
    /// None otherwise
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized
    {
        if self.is_empty() {
            return None
        }

        let hash_val = HashMap::<K,V>::calculate_hash(&key, self.buckets.len());
        for (index,(y,_)) in self.buckets[hash_val].iter().enumerate(){

            if *y == self.buckets[hash_val][index].0{
                self.len -= 1;
                return Some(self.buckets[hash_val].swap_remove(index).1);
            }
        }

        None
    }

    /// Takes a reference to a key and looks for a mapping
    /// If there exists a mapping, returns true
    /// Otherwise false
    pub fn contains_key<Q>(&self,key: &Q) -> bool
    where
        Q:Hash + Eq + ?Sized,
        K:Borrow<Q>
    {
        match self.get(key){
            Some(_) => true,
            None => false
        }
    }

    /**
    * DefaultHasher gives implementation of a Hasher
    * i32 implements hash trait, allowing this value to be fed to the hasher
    * the finish() will return the hashed value
    *
    * DefaultHasher is guaranteed to be the same, if created through new
    */
    fn calculate_hash<Q>(key: &Q,length:usize) -> usize
    where
        K: Borrow<Q>,
        Q: Hash + ?Sized
    {
        let mut x = DefaultHasher::new();
        key.hash(&mut x);
        (x.finish() % length as u64) as usize
    }
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_new(){
        let hashmap = HashMap::<char,i32>::new();
        assert_eq!(hashmap.buckets.len(),0);
    }

    #[test]
    fn test_add(){
        let mut map = HashMap::new();
        assert_eq!(map.len, 0);
        assert!(map.is_empty());
        map.put("test", 42);
        assert_eq!(map.len, 1);
        assert!(!map.is_empty());
        assert_eq!(map.get("test"),Some(&42));
        assert_eq!(map.remove("ag"), None);
        assert_eq!(map.len, 1);
        assert_eq!(map.get("h"), None);
        map.remove("test");
        assert!(map.is_empty());
    }

    #[test]
    fn test_remove(){
        let mut hashmap = HashMap::<char,i32>::new();
        assert_eq!(hashmap.remove(&'h'),None);
        hashmap.put('h',10);
        assert_eq!(hashmap.get(&'h'),Some(&10));
        assert_eq!(hashmap.remove(&'h'),Some(10));
        assert_eq!(hashmap.remove(&'h'),None);

    }

    #[test]
    fn test_iter(){

        let mut hashmap = HashMap::<char,i32>::new();
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

    #[test]
    fn test_contains(){

        let mut hashmap = HashMap::<char,i32>::new();
        hashmap.put('a',1);
        hashmap.put('b',2);
        hashmap.put('c',3);

        assert!(hashmap.contains_key(&'a'));
        assert!(hashmap.contains_key(&'b'));
        assert!(hashmap.contains_key(&'c'));
    }

}
