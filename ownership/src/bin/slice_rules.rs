fn main() {
    let slice;
    {
        let arr = [1, 2, 3, 4, 5];
        slice = &arr[0..5];
        println!("array: {:?}", slice);
    }

    /* println!("array: {:?}", slice);
     * this will give a dangling error
     * */

    let mut s = [1, 2, 3, 4, 5];

    let slice1 = &s[0..5];
    let slice2 = &s[0..5];
    println!("{:?}, {:?}!", slice1, slice2);

    /*
     * multiple immutables can exist
     * only one mutable can exist once
     * both can't exist together
     * immutable should be used before making mutable
     * */

    let slice3 = &mut s[0..5];
    slice3[1] = 8;
    println!("After modifying: {:?}", slice3);

    /*
     * println!("{:?}, {:?}!", slice1, slice2);
     * This won't work as it can't be mutable and immutable at the same time
     * */
}
